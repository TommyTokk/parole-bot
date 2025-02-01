include!("../assets/valid_guesses.rs");
include!("../assets/wordlist.rs");

use std::collections::HashMap;

pub struct Solver {
    pub words: Vec<String>,
    pub valid_guesses: Vec<String>,
    pub word_length: usize,
    pub word_frequencies: HashMap<String, f64>,
}

impl Solver {
    pub fn new() -> Self {
        Solver {
            words: WORDS.iter().map(|&s| s.to_string()).collect(),
            valid_guesses: VALID_GUESSES.iter().map(|&s| s.to_string()).collect(),
            word_length: 5,
            word_frequencies: HashMap::new(),
        }
    }

    pub fn get_next_possible_words(&self, word: &str, color_state: &str) -> Vec<String> {
        // For now, return the color state 
        let mut tmp:Vec<String> = Vec::new();
        tmp.push(color_state.to_string());
        tmp
    }
}



