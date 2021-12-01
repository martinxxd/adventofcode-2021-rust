mod days;

fn main() {
    // https://adventofcode.com/2021
    println!("Advent of Code!");
}

mod utils {
    use std::fs;

    pub(crate) fn read_file(filename: String) -> Vec<String> {
        let contents = fs::read_to_string(filename).unwrap_or("".to_string());
        contents.lines().map(|c| c.to_string()).collect()
    }
}
