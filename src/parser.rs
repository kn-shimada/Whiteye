use crate::lexer::Lexer;
use crate::token::{Token, TokenKind};
use crate::ast::Ast;
use anyhow::Result;

pub struct Parser<'a> {
    pub lexer: &'a mut Lexer,
    // 現在解析中のtoken
    pub cur: Token,
    // 次に解析するtoken
    pub peek: Token,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer) -> Result<Self> {
        Ok(Self {
            cur: lexer.lex()?,
            peek: lexer.lex()?,
            lexer,
        })
    }

    pub fn parse(&self) {
        match self.cur.kind {
            TokenKind::Number(n) => self.parse_number(n),
            TokenKind::Plus => todo!(),
            TokenKind::Minus => todo!(),
            TokenKind::Asterisk => todo!(),
            TokenKind::Slash => todo!(),
            TokenKind::LParen => todo!(),
            TokenKind::RParen => todo!(),
        };
    }

    fn parse_number(&self, n: isize) -> Box<Ast>{
        Box::new(Ast::Number(n));
        next_token()
    }

    fn _next_token(&mut self) -> Result<()> {
        self.cur = self.peek.clone();
        self.peek = self.lexer.lex()?;
        Ok(())
    }
}

fn next_token() -> Box<Ast> {
    todo!()
}
