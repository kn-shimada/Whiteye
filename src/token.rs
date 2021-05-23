#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Number(isize),
    Plus,
    Minus,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub raw_input: String,
}