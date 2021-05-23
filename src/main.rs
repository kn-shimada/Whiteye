use my_language::lexer::Lexer;
use std::{
    fs,
    boxed::Box,
    error::Error
};

fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let input = fs::read_to_string("program.txt").expect("Failed to read the file");
    let mut l = Lexer::new(&input);
    while l.pos < input.len() {
        println!("{:?}", l.lex()?);
    }
    Ok(())
}