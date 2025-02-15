//! A program that solves the NYT spelling bee game.
//!
//! This program uses a generic dictionary and takes
//! as input the seven letters NYT provides (from the user)
//! to filter words and print a list of words that __may__
//! solve the puzzle. The NYT uses a customized dictionary,
//! so not all solutions provided by rusty bee will be valid
//! answers. However, it should provie enough valid solutions
//! that you can win the game.
//!
//! If you're not cheatin' you're not tryin'
//!
//! The Dictionary os provided, but you can provide your own
//! dictionary file (should be a txt file with one word per line)
//! by changing the appropriate file path.

use std::collections::HashSet;
use text_io::read;

// path to dictionary txt file. See documentation to change.
static DICTIONARY_WORDS: &str = include_str!("../assets/words.txt");

/// Create the dictionary of words as a `Vec<String>`
fn create_string(dict_words: &str, required_letter: &String) -> Vec<String> {
    let initial_dictionary: Vec<String> = dict_words.lines().map(String::from).collect();

    let mut processed_dictionary = Vec::new();
    for word in initial_dictionary {
        // filter out words less than 3 letters long and
        // filter out words without the required letter.
        if word.len() > 3 && word.contains(&required_letter.as_str()) {
            processed_dictionary.push(word);
        }
    }
    processed_dictionary
}

/// Gets the center letter required to be in all the
/// solution words.
fn get_magic_letter() -> String {
    print!("Please enter the center letter: ");
    let magic_letter: String = read!();
    magic_letter
}

/// Gets the rest of the letters required to be in the
/// solutions.
fn get_required_letters() -> Vec<String> {
    print!("\nEnter the required letters, without spaces or commas: ");

    let required_letter_raw: String = read!();

    let mut required_letters_processed: Vec<String> =
        required_letter_raw.split("").map(String::from).collect();

    required_letters_processed.remove(0); //remove empty string at beginning
    required_letters_processed.pop(); //remove empty string at start
    required_letters_processed //return required
}

/// Process the required letters and words to provide the
/// user with solutions.
fn process_solutions(required_letters_final: Vec<String>, words: Vec<String>) -> Vec<String> {
    // Convert the vector of letters into a HashSet of characters for quick lookup.
    let allowed_letters: HashSet<char> = required_letters_final
        .iter()
        .flat_map(|s| s.chars())
        .collect();

    // Filter words, retaining only those composed entirely of allowed letters.
    words
        .into_iter()
        .filter(|word| word.chars().all(|c| allowed_letters.contains(&c)))
        .collect()
}

fn main() {
    // get the magic letter
    let required_letter = get_magic_letter();
    println!("The required letter is: {}", required_letter);

    // create a dictionary of words which are longer than 3 letters
    // and only contain the required letter
    let words = create_string(DICTIONARY_WORDS, &required_letter);

    // get the other required letters
    let mut required_letter_set = get_required_letters();
    required_letter_set.push(required_letter);

    // process and create the solution words.
    let solution_set = process_solutions(required_letter_set, words);

    //print the solutions
    for solution in solution_set {
        println!("{}", solution);
    }
}
