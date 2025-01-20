#![allow(unused)]

use std::fs::read_to_string;
use std::path::Path;
use text_io::read;

static DICTIONARY_WORDS: &str = include_str!("../assets/words.txt");

/// Create the dictionary of words as a Vec<String>
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
}

//  TODO:
//  Add features:
//  - [X] create a Vec<String> of words instead of one long String
//  - [X] read in letter required in all words
//  - [X] read in letter set with which to make solution words.
//      - [ ] Make a more ergonomic input system by allowing more than one letter at one time
//  - [ ] iterate over words to select solutions.
//  - [ ] handle different file paths for Windows
