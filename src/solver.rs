include!("../assets/valid_guesses.rs");
include!("../assets/wordlist.rs");
include!("../assets/letter_frequencies.rs");

use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;

// Logger function to write messages to a log file
fn log_to_file(message: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("debug.log")
        .unwrap();
    writeln!(file, "{}", message).unwrap();
}

#[derive(Clone)]
pub struct Solver {
    pub words: Vec<String>,
    //pub valid_guesses: Vec<String>,
    pub word_length: usize,
    pub word_frequencies: HashMap<char, f64>,
    pub previous_words: Vec<(String, String)>
}

impl Solver {
    pub fn new() -> Self {
        let words_copy: Vec<String> = WORDS.iter().map(|&s| s.to_string()).collect();
        Solver {
            words: words_copy.clone(),
            //valid_guesses: VALID_GUESSES.iter().map(|&s| s.to_string()).collect(),
            word_length: 5,
            word_frequencies: LETTER_FREQUENCIES.iter().copied().collect(),
            previous_words: Vec::new()
        }
    }

    /// Given a guessed word and the feedback from comparing it to a candidate answer,
    /// returns a string pattern where:
    /// - 'G' indicates the letter is in the correct position (green),
    /// - 'Y' indicates the letter is present but in a wrong position (yellow),
    /// - 'R' indicates the letter is absent (black).
    pub fn get_feedback_pattern(&self, guess: &str, answer: &str) -> String {
        // Convert strings to vectors of characters.
        let mut pattern = vec!['R'; self.word_length];
        let mut answer_chars: Vec<char> = answer.chars().collect();
        let guess_chars: Vec<char> = guess.chars().collect();

        // First pass: mark greens.
        for i in 0..self.word_length {
            if guess_chars[i] == answer_chars[i] {
                pattern[i] = 'G';
                // Mark this letter as used.
                answer_chars[i] = '*';
            }
        }
        // Second pass: mark yellows.
        for i in 0..self.word_length {
            if pattern[i] == 'G' {
                continue;
            }
            if let Some(pos) = answer_chars.iter().position(|&c| c == guess_chars[i]) {
                pattern[i] = 'Y';
                // Mark the matched letter as used.
                answer_chars[pos] = '*';
            }
        }
        pattern.into_iter().collect()
    }

    /// Calculates the expected information gain (entropy) of making a given guess,
    /// given the current candidate answers. It does this by:
    /// 1. Simulating the feedback pattern for each candidate answer.
    /// 2. Building a distribution over these patterns.
    /// 3. Computing the entropy of that distribution.
    pub fn calculate_expected_entropy(&self, guess: &str, candidate_answers: &[String]) -> f64 {
        let total = candidate_answers.len() as f64;
        let mut pattern_counts: HashMap<String, usize> = HashMap::new();

        // For each candidate answer, simulate the feedback pattern.
        for answer in candidate_answers {
            let pattern = self.get_feedback_pattern(guess, answer);
            *pattern_counts.entry(pattern).or_insert(0) += 1;
        }

        // Compute the entropy: H = - Σ p(pattern) log₂(p(pattern))
        let mut entropy = 0.0;
        for &count in pattern_counts.values() {
            let p = count as f64 / total;
            entropy -= p * p.log2();
        }
        entropy
    }

    //TODO: Check for a possible bug in word filterig

    /// Filters the words based on the absent characters (R), correctly placed characters (G),
    /// and mis-placed characters (Y) as per the input feedback.
    pub fn filter_words(
        &self,
        words: &[String],
        absent_chars: &Vec<(char, usize)>,
        present_chars: &Vec<(char, usize)>,
        wrong_placed_chars: &Vec<(char, usize)>
    ) -> Vec<String> {
        let filtered_words: Vec<String> = words.iter().filter(|word| {
            // Check absent characters.
            for (c, _) in absent_chars {
                if word.contains(*c) {
                    return false;
                }
            }
            // Check correctly placed characters.
            for (c, pos) in present_chars {
                if word.chars().nth(*pos).unwrap_or(' ') != *c {
                    return false;
                }
            }
            // Check wrongly placed characters.
            for (c, pos) in wrong_placed_chars {
                if word.chars().nth(*pos).unwrap_or(' ') == *c || !word.contains(*c) {
                    return false;
                }
            }
            true
        }).cloned().collect();

        // Log the current word list after filtering.
        self.print_word_list();

        filtered_words
    }

    /// Returns the current word list as a log entry.
    pub fn print_word_list(&self) {
        let separator = " | ";
        let word_list = self.words.join(separator);
        log_to_file(&format!("Current word list: {}", word_list));
    }

    /// Given a guess and the feedback state (using 'R', 'G', 'Y') for each letter,
    /// first filters the word list and then calculates the expected entropy for each candidate word.
    /// It returns the candidate words sorted by their expected entropy (information gain) in descending order.
    pub fn get_next_possible_words(&self, word: &str, color_state: &str) -> Vec<String> {
        let mut absent_chars: Vec<(char, usize)> = Vec::new();
        let mut present_chars: Vec<(char, usize)> = Vec::new();
        let mut wrong_placed_chars: Vec<(char, usize)> = Vec::new();

        for (i, c) in word.chars().enumerate() {
            let state = color_state.chars().nth(i).unwrap();
            if state == 'R' {
                absent_chars.push((c, i));
            } else if state == 'G' {
                present_chars.push((c, i));
            } else if state == 'Y' {
                wrong_placed_chars.push((c, i));
            }
        }

        // First filter the words using the provided feedback.
        let filtered_words = self.filter_words(&self.words, &absent_chars, &present_chars, &wrong_placed_chars);
        let mut word_entropy: Vec<(String, f64)> = Vec::new();

        // Now, for each filtered candidate, compute its expected information gain
        // when used as a guess against the current candidate pool.
        for candidate in &filtered_words {
            let entropy = self.calculate_expected_entropy(candidate, &filtered_words);
            word_entropy.push((candidate.to_string(), entropy));
        }

        // Sort the candidates by descending entropy (higher expected info gain first).
        word_entropy.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        word_entropy.into_iter().map(|(word, _)| word).collect()
    }

    pub fn add_used_word(&mut self, word: &str, color_state: &str) {
        self.previous_words.push((word.to_string(), color_state.to_string()));

        self.words.retain(|w| w != word);

        log_to_file(&format!("Word '{}' used. Remaining words: {}", 
                           word, self.words.len()));
    }
}
