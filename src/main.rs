use std::fs;

fn main() {
    let input = fs::read_to_string("program.txt").expect("Failed to read the file");
    println!("{}", input);
}
