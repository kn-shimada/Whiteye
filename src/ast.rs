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
    UPlus,
    UMinus,
}

#[derive(Debug, PartialEq)]
pub enum AssignmentOpKind {
    AEqual,
    AAdd,
    ESub,
    EMul,
    EDiv,
    EExp,
}

#[derive(Debug, PartialEq)]
pub enum VariableType {
    Int,
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
        expr: Box<Ast>,
    },

    Variable {
        name: String,
        data_type: VariableType,
        operator: AssignmentOpKind,
        expr: Box<Ast>,
    },
}
