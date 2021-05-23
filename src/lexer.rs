#[derive(Debug, Clone, PartialEq)]
pub struct Lexer {
    pub input: Vec<char>,
    pub pos: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self {
            input: input.chars().collect(),
            pos: 0,
        }
    }

    pub fn lex(&self) {
        println!("{:?}", self.input);
    }

    pub fn cur(&self) -> Option<&char> {
        self.input.get(self.pos)
    }

    pub fn peek(&self) -> Option<&char> {
        self.input.get(self.pos + 1)
    }

    pub fn next(&mut self) {
        self.pos += 1;
    }
}
