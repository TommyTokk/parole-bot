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

pub struct Solver {
    pub words: Vec<String>,
    //pub valid_guesses: Vec<String>,
    pub word_length: usize,
    pub word_frequencies: HashMap<char, f64>,
}

impl Solver {
    pub fn new() -> Self {
        let words_copy: Vec<String> = WORDS.iter().map(|&s| s.to_string()).collect();
        Solver {
            words: words_copy.clone(),
            //valid_guesses: VALID_GUESSES.iter().map(|&s| s.to_string()).collect(),
            word_length: 5,
            word_frequencies: LETTER_FREQUENCIES.iter().copied().collect(),
        }
    }

    pub fn get_next_possible_words(&self, word: &str, color_state: &str) -> Vec<String> {
        let mut absent_chars: Vec<(char, usize)> = Vec::new();
        let mut present_chars: Vec<(char, usize)> = Vec::new();
        let mut wrong_placed_chars: Vec<(char, usize)> = Vec::new();

        for (i, c) in word.chars().enumerate() {
            if color_state.chars().nth(i).unwrap() == 'R' {
                absent_chars.push((c, i));
            } else if color_state.chars().nth(i).unwrap() == 'G' {
                present_chars.push((c, i));
            } else if color_state.chars().nth(i).unwrap() == 'Y' {
                wrong_placed_chars.push((c, i));
            }
        }

        // Call the filter_words method to get the filtered list of words
        self.filter_words(&self.words, &absent_chars, &present_chars, &wrong_placed_chars)
    }

    pub fn calculate_char_information(&self, char: char) -> f64 {
        //Calculate the information of the character
        let char_probability = self.word_frequencies[&char];
        -char_probability.log2()
    }

    pub fn calculate_word_entropy(&self, word: &str) -> f64 {
        //Calculate the entropy of the word
        let mut entropy = 0.0;
        for c in word.chars() {
            entropy += self.calculate_char_information(c);
        }
        entropy
    }

    pub fn filter_words(
        &self,
        words: &[String],
        absent_chars: &Vec<(char, usize)>,
        present_chars: &Vec<(char, usize)>,
        wrong_placed_chars: &Vec<(char, usize)>
    ) -> Vec<String> {
        let filtered_words: Vec<String> = words.iter().filter(|word| {
            // Check absent characters
            for (c, _) in absent_chars {
                if word.contains(*c) {
                    //log_to_file(&format!("Filtering out word: {} because it contains absent character: {}", word, c));
                    return false; // Word contains an absent character
                }
            }

            // Check present characters
            for (c, pos) in present_chars {
                if word.chars().nth(*pos).unwrap_or(' ') != *c {
                    //log_to_file(&format!("Filtering out word: {} because character: {} is not in the correct position: {}", word, c, pos));
                    return false; // Character is not in the correct position
                }
            }

            // Check wrong placed characters
            for (c, pos) in wrong_placed_chars {
                if word.chars().nth(*pos).unwrap_or(' ') == *c || !word.contains(*c) {
                    //log_to_file(&format!("Filtering out word: {} because character: {} is either in the wrong position or not present", word, c));
                    return false; // Character is either in the wrong position or not present
                }
            }

            true // Word passed all checks
        }).cloned().collect();

        // Print the current word list after filtering
        self.print_word_list();

        filtered_words // Return the filtered words
    }

    pub fn print_word_list(&self) {
        let separator = " | "; // Define your separator here
        let word_list = self.words.join(separator);
        log_to_file(&format!("Current word list: {}", word_list)); // Log to the debug file
    }
}
