use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    // create a path to the dictionary file.
    let path = Path::new("assets/words.txt");
    let display = path.display();

    // Open the path

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the dictionary into a string
    let mut words = String::new();
    match file.read_to_string(&mut words) {
        Err(why) => panic!("Couldn't read {}: {}", display, why),
        Ok(_) => print!("{}", words),
    }
}
