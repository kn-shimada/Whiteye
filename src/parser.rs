use crate::lexer::Lexer;
use crate::token::Token;
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

    pub fn next_token(&mut self) -> Result<()> {
        self.cur = self.peek.clone();
        self.peek = self.lexer.lex()?;
        Ok(())
    }
}
