include!("../assets/valid_guesses.rs");
include!("../assets/wordlist.rs");
include!("../assets/letter_frequencies.rs");

use std::collections::HashMap;

pub struct Solver {
    pub words: Vec<String>,
    pub valid_guesses: Vec<String>,
    pub word_length: usize,
    pub word_frequencies: HashMap<char, f64>,
}

impl Solver {
    pub fn new() -> Self {
        Solver {
            words: WORDS.iter().map(|&s| s.to_string()).collect(),
            valid_guesses: VALID_GUESSES.iter().map(|&s| s.to_string()).collect(),
            word_length: 5,
            word_frequencies: LETTER_FREQUENCIES.iter().copied().collect(),
        }
    }

    pub fn get_next_possible_words(&self, word: &str, color_state: &str) -> Vec<String> {
        let mut possible_words = self.remove_absent_letters(word, color_state);
        possible_words = self.remove_wrong_place_letters(word, color_state);

        let mut word_scores: Vec<(String, f64)> = possible_words
            .iter()
            .map(|w| (w.clone(), self.calculate_word_entropy(w)))
            .collect();
        
        word_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        word_scores.iter()
            .take(3)
            .map(|(w, _)| w.clone())
            .collect()
    }
    
    pub fn remove_absent_letters(&self, word: &str, color_state: &str) -> Vec<String> {
        //Remove the words that contain the red letters
        let mut possible_words = self.words.clone();
        let colors: Vec<char> = color_state.chars().collect();
        for (i, c) in word.chars().enumerate() {
            if colors[i] == 'R' {
                possible_words.retain(|w| !w.chars().nth(i).unwrap_or(' ').eq(&c));
            }
        }
        possible_words
    }

    pub fn remove_wrong_place_letters(&self, word: &str, color_state: &str) -> Vec<String> {
        //Remove the words that contain the yellow letters in the wrong place
        let mut possible_words = self.words.clone();
        let colors: Vec<char> = color_state.chars().collect();
        for (i, c) in word.chars().enumerate() {
            if colors[i] == 'Y' {
                possible_words.retain(|w| w.chars().nth(i).unwrap_or(' ').eq(&c));
            }
        }
        possible_words
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
}



