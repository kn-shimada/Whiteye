#[derive(Debug, Clone, PartialEq)]
pub struct Lexer {
  pub input: Vec<char>,
  pub pos: usize
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self {
            input: input.chars().collect(),
            pos: 0
        }
    }

    pub fn lex(&self) {
        println!("{:?}", self.input);
    }
}