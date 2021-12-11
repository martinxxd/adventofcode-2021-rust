#![allow(dead_code)]

mod days;

fn main() {
    // https://adventofcode.com/2021
    println!("Advent of Code!");
}

mod utils {
    use std::fs;

    pub(crate) fn read_file(filename: String) -> String {
        fs::read_to_string(filename).unwrap_or("".to_string())
    }

    pub(crate) fn read_file_lines(filename: String) -> Vec<String> {
        let contents = read_file(filename);
        contents.lines().map(|c| c.to_string()).collect()
    }
}
