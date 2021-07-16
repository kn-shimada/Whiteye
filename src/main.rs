use anyhow::Result;
use std::fs;
use whiteye::parser::parse;

fn main() -> Result<()> {
    let input = fs::read_to_string("program.txt").expect("Failed to read the file");
    println!("{:?}", &input);
    println!("{:?}", parse(&input));
    Ok(())
}
