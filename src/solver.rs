include!("../assets/valid_guesses.rs");
include!("../assets/wordlist.rs");

use std::collections::HashMap;

pub const letter_frequencies: HashMap<char, f64> = [
        ('A', 0.1085),
        ('B', 0.0105),
        ('C', 0.0430),
        ('D', 0.0339),
        ('E', 0.1149),
        ('F', 0.0101),
        ('G', 0.0165),
        ('H', 0.0143),
        ('I', 0.1018),
        ('L', 0.0570),
        ('M', 0.0287),
        ('N', 0.0702),
        ('O', 0.0997),
        ('P', 0.0296),
        ('Q', 0.0045),
        ('R', 0.0619),
        ('S', 0.0548),
        ('T', 0.0697),
        ('U', 0.0316),
        ('V', 0.0175),
        ('Z', 0.0085),
    ]
    .iter()
    .cloned()
    .collect(); 

pub struct Solver {
    pub words: Vec<String>,
    pub valid_guesses: Vec<String>,
    pub word_length: usize,
    pub word_frequencies: HashMap<String, f64>,
}



