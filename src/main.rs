use std::fs::read_to_string;
use std::path::Path;

fn read_lines(filename: &Path) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn main() {
    // create a path to the dictionary file.
    let path = Path::new("assets/words.txt");

    let words = read_lines(path);

    for word in words {
        println!("{}", word);
    }
}

//  TODO:
//  Add features:
//  - [ ] read in letter required in all words
//  - [ ] read in letter set with which to make solution words.
//  - [ ] iterate over words to select solutions.
