#![allow(unused)]

use std::fs::read_to_string;
use std::path::Path;
use text_io::read;

static DICTIONARY_WORDS: &str = include_str!("../assets/words.txt");

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

fn main() {
    // create a path to the dictionary file.
    let path = Path::new("assets/words.txt");

    // create the dictionary of words (a Vec<String>)
    //let words = read_lines(path);

    let words = create_string(DICTIONARY_WORDS);

    for word in words {
        println!("{}", word);
    }

    // get the magic letter
    let required_letter = get_magic_letter();
    println!("The required letter is: {}", required_letter);
}

//  TODO:
//  Add features:
//  - [X] create a Vec<String> of words instead of one long String
//  - [X] read in letter required in all words
//  - [ ] read in letter set with which to make solution words.
//  - [ ] iterate over words to select solutions.
//  - [ ] handle different file paths for Windows
