use crate::token::{Token, TokenKind};
use anyhow::Result;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq)]
pub struct Lexer {
    pub input: Vec<char>, //解析する文字列の取得
    pub pos: usize,       //解析しているインデックス番号
}

impl Lexer {
    //初期化
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            pos: 0,
        }
    }

    pub fn lex(&mut self) -> Result<Token> {
        //空白をスキップ
        while self.cur().is_ok() && self.cur().unwrap().is_whitespace() {
            self.next()?;
        }

        //数値の取得
        if self.cur().is_ok() && self.cur().unwrap().is_ascii_digit() {
            let mut num_str = String::from("");
            while self.cur().is_ok() && self.cur().unwrap().is_ascii_digit() {
                num_str.push(*self.cur().unwrap());
                self.next()?;
            }
            return Ok(Token {
                kind: TokenKind::Number(num_str.parse::<isize>().unwrap()),
                raw_input: num_str,
            });
        }

        //演算子の取得
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
            _ => return Err(LexerError::UnexpectedCharacterError(*self.cur().unwrap()).into()),
        }
    }

    //解析中の文字列
    fn cur(&self) -> Result<&char> {
        let raw_char = self.input.get(self.pos);
        if raw_char.is_none() {
            return Err(LexerError::InvalidPosition(self.pos).into());
        }
        Ok(raw_char.unwrap())
    }

    //解析するインデックスを進める
    fn next(&mut self) -> Result<()> {
        if self.pos + 1 > self.input.len() {
            return Err(LexerError::MaximumPosition(self.pos).into());
        }
        self.pos += 1;
        Ok(())
    }
}

//error処理
#[derive(Debug, Error)]
pub enum LexerError {
    #[error("Unexpected Character: {0}")]
    UnexpectedCharacterError(char),
    #[error("Invalid position: {0}")]
    InvalidPosition(usize),
    #[error("The position is maximum: {0}")]
    MaximumPosition(usize),
}
