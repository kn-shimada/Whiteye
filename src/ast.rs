#[derive(Debug, PartialEq)]
pub enum OpKind {
    SingleTermOp,
    Add,
    Sub,
    Mul,
    Div,
    Exp,
}

#[derive(Debug, PartialEq)]
pub enum Ast {
    Number(isize),

    Expr {
        left: Box<Ast>,
        operator: OpKind,
        right: Box<Ast>,
    },

    Term {
        operator: OpKind,
        right: Box<Ast>,
    }
}
