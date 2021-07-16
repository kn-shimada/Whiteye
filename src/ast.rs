#[derive(Debug, PartialEq)]
pub enum OpKind {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, PartialEq)]
pub enum Ast {
    Number(isize),
    //例: 1 + 2
    BinaryOp(OpKind, Box<Ast>, Box<Ast>)
}
