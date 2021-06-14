#[derive(Debug, PartialEq)]
pub enum OpKind {
    Add,
    Sub,
    Mul,
    Div
}

#[derive(Debug, PartialEq)]
pub enum Ast {
    Number(isize),

    //例: 1 + 2
    Expr {
        left: Box<Ast>,
        operator: OpKind,
        right: Box<Ast>,
    },
}


