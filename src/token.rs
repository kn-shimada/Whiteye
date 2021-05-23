#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Number(isize),
    Plus,
    Minus,
    Eof,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub raw_input: String,
}
