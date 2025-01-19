#![allow(unused)]

use std::fs::read_to_string;
use std::path::Path;
use text_io::{read, scan};

static DICTIONARY_WORDS: &str = include_str!("../assets/words.txt");

/// Create the dictionary of words as a Vec<String>
fn create_string(dict_words: &str) -> Vec<String> {
    dict_words.lines().map(String::from).collect()
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

    // let required_letters_processed: Vec<String> = required_letters_raw.split(' ').map(String::from).collect();
    required_letters_processed
}

fn main() {
    // create a path to the dictionary file.
    let path = Path::new("assets/words.txt");

    // create the dictionary of words (a Vec<String>)
    //let words = read_lines(path);

    let words = create_string(DICTIONARY_WORDS);

    // get the magic letter
    let required_letter = get_magic_letter();
    println!("The required letter is: {}", required_letter);

    // get the other required letters
    let other_required_letters = get_required_letters();
    println!("{:?}", other_required_letters);
}

//  TODO:
//  Add features:
//  - [X] create a Vec<String> of words instead of one long String
//  - [X] read in letter required in all words
//  - [X] read in letter set with which to make solution words.
//      - [ ] Make a more ergonomic input system by allowing more than one letter at one time
//  - [ ] iterate over words to select solutions.
//  - [ ] handle different file paths for Windows
