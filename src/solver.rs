include!("../assets/valid_guesses.rs");
include!("../assets/wordlist.rs");
include!("../assets/letter_frequencies.rs");

use std::collections::HashMap;

pub struct Solver {
    pub words: Vec<String>,
    //pub valid_guesses: Vec<String>,
    pub word_length: usize,
    pub word_frequencies: HashMap<char, f64>,
}

impl Solver {
    pub fn new() -> Self {
        Solver {
            words: WORDS.iter().map(|&s| s.to_string()).collect(),
            //valid_guesses: VALID_GUESSES.iter().map(|&s| s.to_string()).collect(),
            word_length: 5,
            word_frequencies: LETTER_FREQUENCIES.iter().copied().collect(),
        }
    }

    pub fn get_next_possible_words(&self, word: &str, color_state: &str) -> Vec<String> {
        todo!()

        
    }

    fn remove_absent_letters(&self, word: &str, wrong_characters: Vec<char>, absent: Vec<usize>) -> Vec<String> {
        let mut res: Vec<String> = Vec::new();
        let word_chars: Vec<char> = word.chars().collect();
        let correct_positions: Vec<(usize, char)> = word_chars.iter()
            .enumerate()
            .filter(|(i, _)| !absent.contains(i))
            .map(|(i, &c)| (i, c))
            .collect();

        for w in self.words.iter() {
            if w.chars().all(|c| !wrong_characters.contains(&c)) && 
               correct_positions.iter().all(|(pos, c)| w.chars().nth(*pos).unwrap() == *c) {
                res.push(w.clone());
            }
        }
        res
    }

    fn get_correct_positions(&self, word_chars: &[char], color_chars: &[char]) -> Vec<(usize, char)> {
        word_chars.iter()
            .zip(color_chars.iter())
            .enumerate()
            .filter(|(_, (_, &c))| c == 'G')
            .map(|(i, (&w, _))| (i, w))
            .collect()
    }

    fn get_misplaced_positions(&self, word_chars: &[char], color_chars: &[char]) -> Vec<(usize, char)> {
        word_chars.iter()
            .zip(color_chars.iter())
            .enumerate()
            .filter(|(_, (_, &c))| c == 'Y')
            .map(|(i, (&w, _))| (i, w))
            .collect()
    }

    fn get_absent_letters(&self, word_chars: &[char], color_chars: &[char]) -> Vec<char> {
        word_chars.iter()
            .zip(color_chars.iter())
            .filter(|(_, &c)| c == 'R')
            .map(|(&w, _)| w)
            .collect()
    }

    fn is_word_valid(&self, word: &str, 
                     correct_positions: &[(usize, char)],
                     misplaced_positions: &[(usize, char)],
                     absent_letters: &[char]) -> bool {
        let w_chars: Vec<char> = word.chars().collect();
        
        // Check correct positions (green)
        let correct_pos_match = correct_positions.iter()
            .all(|&(pos, c)| w_chars[pos] == c);
            
        // Check misplaced letters (yellow)
        let misplaced_match = misplaced_positions.iter()
            .all(|&(pos, c)| w_chars.contains(&c) && w_chars[pos] != c);
            
        // Check absent letters (red/grey)
        let absent_match = absent_letters.iter()
            .all(|&c| {
                let is_elsewhere = correct_positions.iter().any(|&(_, cc)| cc == c) ||
                                 misplaced_positions.iter().any(|&(_, cc)| cc == c);
                !w_chars.contains(&c) || is_elsewhere
            });
            
        correct_pos_match && misplaced_match && absent_match
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
