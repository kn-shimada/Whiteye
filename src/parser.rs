use crate::ast::Expr;
use crate::lexer::Lexer;


// 構造体
pub struct Perser {
    // 字句解析器
    pub lexer: Lexer,
    // 現在解析中の字句
    pub cur: Option<Token>,
    // 次に解析する字句
    pub peek: Option<Token>,
}

// 優先度
#[derive(PartialOrd, PartialEq)]
enum Precedence {
    // 最低
    LOWEST,
    // "+", "-"
    SUM,
    // 前置演算子
    PREFIX,
}

impl parser {
    // 初期化
    pub fn new(mut lexer: Lexer) -> Self {
        Self{
            lexer: Lexer,
            cur: lexer.lex(),
            peek: lexer.lex(),
        }
    }

    // 優先度の定義
    fn token_precedence(token: &TokenKind) -> Precedence {
        match token {
            TokenKind::Plus | TokenKind::Minus => Precedence::SUM,
            _ => Precedence::LOWEST,
        }
    }

    // 構文木の葉の要素の解析
    fn parse_prefix(&mut self) -> Option<Box<Expr>> {
        match self.cur.as_ref()? {
            TokenKind::Minus => self.parse_minus(),
            TokenKind::Number(_) => self.parse_number(),
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
            Some(TokenKind::Number(n)) => Some(Box::new(Expr::Number(*n))),
            _ => None,
        }
    }

    // 優先度を最低に
    fn parse(&mut self) -> Option<Box<Expr>> {
        self.parse_expression(Precedence::LOWEST)
    }

    // 式の解析 
    fn parse_expression(&mut self, precedence: Precedence) -> Option<Box<Expr>> {
        // 葉の要素の解析
        let mut left = self.parse_prefix()?;

        // 中置演算子式の解析を繰り返す
        while self.peek.is_some() && precedence < self.peek_precedence() {
            self.next();
            left = self.parse_infix(left)?;
        }

        return Some(left);
    }

    // 中置演算子式の解析
    fn parse_infix(&mut self, left: Box<Expr>) -> Option<Box<Expr>> {
        let token = self.cur.as_ref()?;
        match token {
            TokenKind::Plus | TokenKind::Minus => {
                self.parse_infix_expression(left)
            }
            _ => Some(left),
        }
    }

    // 中置演算子式の作成
    fn parse_infix_expression(&mut self, left: Box<Expr>) -> Option<Box<Expr>> {
        let token = self.cur.as_ref()?;
        let operator = format!("{:?}", token);
        let precedence = Self::token_precedence(token);
        self.next();
        let right = self.parse_expression(precedence)?;
        return Some(Box::new(Expr::InfixExpr {
            left,
            operator,
            right,
        }));
    }

    // peekの優先度取得
    fn peek_precedence(&self) -> Precedence {
        let token = self.peek.borrow();
        if token.is_none() {
            return Precedence::LOWEST;
        }
        return Self::token_precedence(token.as_ref().unwrap());
    }

    // 解析対象を次の字句に
    pub fn next(&mut self)  -> Self {
        Self{
            cur: lexer.peek.clone(),
            peek: lexer.lexer.lex(),
        }
    }
}