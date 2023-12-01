use std::fs::File;
use std::io::Read;

pub fn file(name: &str) -> String {
    let mut string = String::new();

    File::open(name)
        .expect("Input not found")
        .read_to_string(&mut string)
        .expect("Error while reading file");

    string
}
