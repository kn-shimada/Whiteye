use crate::token::{Token, TokenKind};

#[derive(Debug, Clone, PartialEq)]
pub struct Lexer {
    pub input: Vec<char>,
    pub pos: usize,
}

impl Lexer {
    pub fn new(input: &String) -> Self {
        Self {
            input: input.chars().collect(),
            pos: 0,
        }
    }

    pub fn lex(&mut self) -> Token {
        if self.cur().is_some() && self.cur().unwrap().is_ascii_digit() {
            let mut num_str = String::from("");
            num_str.push(*self.cur().unwrap());
            self.next();
            if self.cur().is_some() && self.cur().unwrap().is_ascii_digit(){
                num_str.push(*self.cur().unwrap());
                self.next();
                return Token{
                    kind: TokenKind::Number(num_str.parse::<isize>().unwrap()),
                    raw_input: num_str,
                };
            }
            return Token{
                kind: TokenKind::Number(num_str.parse::<isize>().unwrap()),
                raw_input: num_str,
            }
        }
        match self.cur() {
            Some('+') => {
                self.next();
                Token{
                    kind: TokenKind::Plus,
                    raw_input: "+".to_string(),
                }
            },
            Some('-') => {
                self.next();
                Token{
                    kind: TokenKind::Minus,
                    raw_input: "-".to_string(),
                }
            },
            None => {
                self.next();
                Token{
                    kind: TokenKind::EOF,
                    raw_input: "".to_string()
                }
            },
            _ => {
                panic!("error");
            }
        }
    }

    fn cur(&self) -> Option<&char> {
        self.input.get(self.pos)
    }

    fn _peek(&self) -> Option<&char> {
        self.input.get(self.pos + 1)
    }

    fn next(&mut self) {
        self.pos += 1;
    }
}
