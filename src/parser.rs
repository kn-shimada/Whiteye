use crate::ast::Expr;
//lexerのuseの仕方が分からん


pub struct Perser {
    pub lexer: Lexer,
    pub cur: Option<Token>,
    pub peek: Option<Token>,
}

impl parser {
    pub fn new(mut lexer: Lexer) -> Self {
        Self{
            lexer: Lexer,
            cur: lexer.lex(),
            peek: lexer.lex(),
        }
    }

    pub fn next(&mut self)  -> Self {
        Self{
            cur: lexer.peek.clone(),
            peek: lexer.lexer.lex(),
        }
    }
}