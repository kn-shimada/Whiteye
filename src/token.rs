#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Number(isize), // 数値
    Plus, // + 
    Minus, // -
    Asterisk, // *
    Slash, // /
    LParen, // (
    RParen, // )
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub raw_input: String,
}
