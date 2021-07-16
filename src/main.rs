use anyhow::Result;
use std::fs;
use std::process::exit;

use whiteye::parser::parse;

fn main() -> Result<()> {
    let input = fs::read_to_string("program.txt").expect("Failed to read the file");
    println!("{:?}", &input);
    let (input, parsed) = parse(&input).unwrap();
    if !input.is_empty() {
        eprintln!("parsing error, input remaining {:?}", input);
        exit(1);
    }

    println!("{:?}", parsed);
    Ok(())
}
