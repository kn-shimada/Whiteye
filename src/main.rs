use anyhow::Result;
use std::fs;
use whiteye::parser::parse_add_sub;

fn main() -> Result<()> {
    let input = fs::read_to_string("program.txt").expect("Failed to read the file");
    println!("{:?}", &input);
    println!("{:?}", parse_add_sub(&input));
    Ok(())
}
