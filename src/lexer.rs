use crate::token::{Token, TokenKind};
use anyhow::Result;
use thiserror::Error;

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

    pub fn lex(&mut self) -> Result<Token> {
        while self.cur().is_ok() && self.cur().unwrap().is_whitespace() {
            self.next()?;
        }

        if self.cur().is_ok() && self.cur().unwrap().is_ascii_digit() {
            let mut num_str = String::from("");
            while self.cur().is_ok() && self.cur().unwrap().is_ascii_digit(){
                num_str.push(*self.cur().unwrap());
                self.next()?;
            }
            return Ok(Token {
                kind: TokenKind::Number(num_str.parse::<isize>().unwrap()),
                raw_input: num_str,
            });
        }

        match self.cur()? {
            '+' => {
                self.next()?;
                Ok(Token {
                    kind: TokenKind::Plus,
                    raw_input: "+".to_string(),
                })
            }
            '-' => {
                self.next()?;
                Ok(Token {
                    kind: TokenKind::Minus,
                    raw_input: "-".to_string(),
                })
            }
            '*' => {
                self.next()?;
                Ok(Token {
                    kind: TokenKind::Asterisk,
                    raw_input: "*".to_string(),
                })
            }
            '/' => {
                self.next()?;
                Ok(Token {
                    kind: TokenKind::Slash,
                    raw_input: "/".to_string(),
                })
            }
            '(' => {
                self.next()?;
                Ok(Token {
                    kind: TokenKind::LParen,
                    raw_input: "(".to_string(),
                })
            }
            ')' => {
                self.next()?;
                Ok(Token {
                    kind: TokenKind::RParen,
                    raw_input: ")".to_string(),
                })
            }
            _ => Err(LexerError::UnexpectedCharacterError(*self.cur().unwrap()))?,
        }
    }

    fn cur(&self) -> Result<&char> {
        let raw_char = self.input.get(self.pos);
        if raw_char.is_none() {
            Err(LexerError::InvalidPosition(self.pos))?;
        }
        Ok(raw_char.unwrap())
    }

    fn next(&mut self) -> Result<()> {
        if self.pos + 1 > self.input.len() {
            return Err(LexerError::MaximumPosition(self.pos))?;
        }
        self.pos += 1;
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum LexerError {
    #[error("Unexpected Character: {0}")]
    UnexpectedCharacterError(char),
    #[error("Invalid position: {0}")]
    InvalidPosition(usize),
    #[error("The position is maximum: {0}")]
    MaximumPosition(usize),
}
