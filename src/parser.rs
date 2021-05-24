use crate::ast::Expr;

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
}