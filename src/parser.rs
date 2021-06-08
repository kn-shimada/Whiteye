use crate::ast::Expr;
use crate::lexer;


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

    // 構文木の葉の要素の解析
    fn parse_prefix(&mut self) -> Option<Box<Expr>> {
        match self.cur.as_ref()? {
            Token::Minus => self.parse_minus(),
            Token::Number(_) => self.parse_number(),
            _ => None,
        }
    }

    // -が来た時の前置演算子式の解析
    fn parse_minus(&mut self) -> Option<Box<Expr>> {
        self.next();
        let number = self.parse_prefix()?;
        return Some(Box::new(Expr::PrefixExpr {
            operator: "Minus".to_string(),
            right: number,
        }));
    }

    // 数値が来た時の解析
    fn parse_number(&mut self) -> Option<Box<Expr>> {
        match self.cur.borrow() {
            Some(Token::Number(n)) => Some(Box::new(Expr::Number(*n))),
            _ => None,
        }
    }

    pub fn next(&mut self)  -> Self {
        Self{
            cur: lexer.peek.clone(),
            peek: lexer.lexer.lex(),
        }
    }
}