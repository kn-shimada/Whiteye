use std::fs;
use my_language::lexer::Lexer;

fn main() {
    let input = fs::read_to_string("program.txt").expect("Failed to read the file");
    let l = Lexer::new(input);
    l.lex();
}
