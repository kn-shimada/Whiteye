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
    ASub,
    AMul,
    ADiv,
    AExp,
}

#[derive(Debug, PartialEq)]
pub enum VariableType {
    Int,
}

#[derive(Debug, PartialEq)]
pub enum Ast {
    Number(isize),

    Variable(String),

    Expr {
        left: Box<Ast>,
        operator: ExprOpKind,
        right: Box<Ast>,
    },

    Monomial {
        operator: UnaryOpKind,
        expr: Box<Ast>,
    },

    VariableDeclaration {
        name: String,
        data_type: VariableType,
        operator: AssignmentOpKind,
        expr: Box<Ast>,
    },

    FunctionCall {
        name: String,
        argument: Box<Ast>,
    },
}
