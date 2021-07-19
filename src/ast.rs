#[derive(Debug, PartialEq)]
pub enum ExprOpKind {
    EAdd,
    ESub,
    EMul,
    EDiv,
    EExp,
}

#[derive(Debug, PartialEq)]
pub enum UnaryOpKind {
    UNone,
    UPuls,
    UMinus,
}

#[derive(Debug, PartialEq)]
pub enum Ast {
    Number(isize),

    Expr {
        left: Box<Ast>,
        operator: ExprOpKind,
        right: Box<Ast>,
    },

    Monomial {
        operator: UnaryOpKind,
        right: Box<Ast>,
    }
}
