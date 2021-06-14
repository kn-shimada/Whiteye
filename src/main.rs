use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
    let _input = fs::read_to_string("program.txt").expect("Failed to read the file");
    Ok(())
}
