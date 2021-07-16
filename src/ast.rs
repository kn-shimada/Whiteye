#[derive(Debug, PartialEq)]
pub enum OpKind {
    Unary,
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

    Monomial {
        operator: OpKind,
        right: Box<Ast>,
    }
}
