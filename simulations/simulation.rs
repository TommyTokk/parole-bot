use std::sync::{Arc, Mutex};
use std::fs::File;

// We need to add rand to Cargo.toml first
// [dependencies]
// rand = "0.8.5"
use rand::seq::SliceRandom;
use csv;
include!("../src/solver.rs");

const DEF_MAX_ITERATIONS: [i16; 3] = [50, 100, 200];

// Define a struct to hold parsed arguments
pub struct Args {
    pub file_path: String,
    pub iterations: Vec<i16>,
}

impl Default for Args {
    fn default() -> Self {
        Self {
            file_path: "results.csv".to_string(),
            iterations: DEF_MAX_ITERATIONS.to_vec(),
        }
    }
}

fn main() {
    // Parse command line arguments
    let args = parse_args(std::env::args().collect());
    
    // Create and prepare the file
    {
        // First create/truncate the file and write the header
        let mut file = File::create(&args.file_path).unwrap();
        writeln!(file, "max_iterations , 1 , 2 , 3 , 4 , 5 , 6 , >6").unwrap();
    }
    
    // Create a mutex-protected file path for thread safety
    let file_path = Arc::new(Mutex::new(args.file_path));
    
    // Use parsed iterations instead of hardcoded values
    let max_iterations = args.iterations;
    let mut handles = vec![];
    
    for max in max_iterations {
        let file_path_clone = Arc::clone(&file_path);
        let handle = std::thread::spawn(move || {
            let classes = simulate_game(max);
            
            // Get a locked reference to the file path
            let path = file_path_clone.lock().unwrap();
            append_to_csv(&classes, max, &path);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("All threads finished. Results written to {}", file_path.lock().unwrap());
}

pub fn parse_args(args: Vec<String>) -> Args {
    let mut result = Args::default();
    let mut i = 1; // Skip program name at args[0]
    
    while i < args.len() {
        match args[i].as_str() {
            "-f" | "--file" => {
                if i + 1 < args.len() {
                    result.file_path = args[i + 1].clone();
                    i += 2;
                } else {
                    eprintln!("Error: Missing file path after -f/--file");
                    std::process::exit(1);
                }
            },
            "-i" | "--iterations" => {
                result.iterations.clear();
                i += 1;
                
                // Collect iteration values until we hit another flag or end of args
                let mut count = 0;
                while i < args.len() && !args[i].starts_with('-') && count < 3 {
                    match args[i].parse::<i16>() {
                        Ok(val) => {
                            result.iterations.push(val);
                            i += 1;
                            count += 1;
                        },
                        Err(_) => {
                            eprintln!("Error: Invalid iteration count '{}'", args[i]);
                            std::process::exit(1);
                        }
                    }
                }
                
                if count == 0 {
                    eprintln!("Error: No iteration counts provided after -i/--iterations");
                    std::process::exit(1);
                }
            },
            _ => {
                eprintln!("Unknown argument: {}", args[i]);
                eprintln!("Usage: {} -f/--file FILE -i/--iterations COUNT [COUNT...]", args[0]);
                std::process::exit(1);
            }
        }
    }
    
    result
}

pub fn get_colorcode(chosen_word: &str, guess: &str) -> String {
    let mut colorcode = String::new();
    for (i, c) in guess.chars().enumerate() {
        //if the character is not in the word
        if !chosen_word.contains(c){
            colorcode.push('R');
        }else{
            //if the character is in the word but not in the right position
            if chosen_word.chars().nth(i).unwrap() != c{
                colorcode.push('Y');
            }else{
                //if the character is in the word and in the right position
                colorcode.push('G');
            }
        }
    }
    colorcode
}

pub fn simulate_game(max_iterations: i16) -> HashMap<String, i16> {
    let mut classes: HashMap<String, i16> = HashMap::from([
        ("1".to_string(), 0),
        ("2".to_string(), 0),
        ("3".to_string(), 0),
        ("4".to_string(), 0),
        ("5".to_string(), 0),
        ("6".to_string(), 0),
        (">6".to_string(), 0),
    ]);
        
    println!("Max iterations: {}", max_iterations);
    
    let mut iteration = 0;
    while iteration < max_iterations {  // Changed <= to <
        // Create a fresh solver for each game
        let mut solver = Solver::new();
        let words: Vec<String> = WORDS.iter().map(|&s| s.to_string()).collect();
        
        let chosen_word = words.choose(&mut rand::thread_rng()).unwrap();
        let mut attempt = 1;
        
        // Use the solver's best opener for the first guess
        let mut guess = "tares".to_string();  // Starting word
        
        while attempt <= 6 {
            // println!("Iteration: {}, Attempt: {}", iteration + 1, attempt);
            // println!("Chosen word: {}, guess: {}", chosen_word, guess);

            if guess == *chosen_word {
                //println!("Found the word: {}", chosen_word);
                // Update the counter for this attempt number
                let class = classes.entry(attempt.to_string()).or_insert(0);
                *class += 1;
                break;
            }

            let colorcode = get_colorcode(chosen_word, &guess);
            //println!("Colorcode: {}", colorcode);

            let res = solver.get_next_possible_words(&guess, &colorcode);

            if res.is_empty() {
                println!("No possible words found");
                break;
            }

            // Get the first word in res
            guess = res[0].clone();
            //println!("Next guess: {}", guess);

            attempt += 1;
        }

        // Handle case where word wasn't found in 6 attempts
        if attempt > 6 {
            println!("Word not found in 6 attempts");
            let class = classes.entry(">6".to_string()).or_insert(0);
            *class += 1;
        }
        
        // Don't forget to increment iteration!
        iteration += 1;
    }

    classes
}

// New function to append to the CSV file
pub fn append_to_csv(classes: &HashMap<String, i16>, max_iterations: i16, file_path: &str) -> String {
    // Open file in append mode
    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(file_path)
        .unwrap();
    
    let mut wtr = csv::WriterBuilder::new()
        .has_headers(false)  // Don't write headers when appending
        .from_writer(file);
    
    // Create required keys list to ensure consistent CSV structure
    let required_keys = vec!["1", "2", "3", "4", "5", "6", ">6"];
    let mut record = vec![max_iterations.to_string()];

    // Process only the required keys in specific order
    for key in &required_keys {
        let count = classes.get(&key.to_string()).unwrap_or(&0);
        record.push(count.to_string());
    }
    
    // Write the record with each value as a separate field
    wtr.write_record(&record).unwrap();
    wtr.flush().unwrap();
    
    format!("Results appended to {}", file_path)
}