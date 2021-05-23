use thiserror::Error;
use crate::token::{Token, TokenKind};

#[derive(Debug, Clone, PartialEq)]
pub struct Lexer {
    pub input: Vec<char>,
    pub pos: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            pos: 0,
        }
    }

    pub fn lex(&mut self) -> Result<Token, LexerError> {
        if self.cur().is_some() && self.cur().unwrap().is_ascii_digit() {
            let mut num_str = String::from("");
            num_str.push(*self.cur().unwrap());
            self.next();
            if self.cur().is_some() && self.cur().unwrap().is_ascii_digit(){
                num_str.push(*self.cur().unwrap());
                self.next();
                return Ok(Token{
                    kind: TokenKind::Number(num_str.parse::<isize>().unwrap()),
                    raw_input: num_str,
                });
            }
            return Ok(Token{
                kind: TokenKind::Number(num_str.parse::<isize>().unwrap()),
                raw_input: num_str,
            })
        }
        match self.cur() {
            Some('+') => {
                self.next();
                Ok(Token{
                    kind: TokenKind::Plus,
                    raw_input: "+".to_string(),
                })
            },
            Some('-') => {
                self.next();
                Ok(Token{
                    kind: TokenKind::Minus,
                    raw_input: "-".to_string(),
                })
            },
            None => {
                self.next();
                Ok(Token{
                    kind: TokenKind::Eof,
                    raw_input: "".to_string()
                })
            },
            _ => {
                return Err(LexerError::UnexpectedCharacterError(*self.cur().unwrap()))
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

#[derive(Debug, Error)]
pub enum LexerError {
    #[error("Unexpected Character: {0}")]
    UnexpectedCharacterError(char),
}