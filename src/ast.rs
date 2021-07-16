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
    Expr {
        left: Box<Ast>,
        operator: OpKind,
        right: Box<Ast>,
    },
}
