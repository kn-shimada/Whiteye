use crate::value::Value;

#[derive(Debug, PartialEq)]
pub enum ValueType {
    Integer,
    Float,
    Bool,
}

#[derive(Debug, PartialEq)]
pub enum ExprOpKind {
    EAdd,
    ESub,
    EMul,
    EDiv,
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
}

#[derive(Debug, PartialEq)]
pub enum ComparisonOpKind {
    CEqual,
    CNot,
    CMore,
    CLess,
    CMoreEqual,
    CLessEqual,
}

#[derive(Debug, PartialEq)]
pub enum LogicalOpKind {
    LAnd,
    LOr,
}

#[derive(Debug, PartialEq)]
pub enum Ast {
    Literal(Value),

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

    ComparisonExpr {
        left: Box<Ast>,
        operator: ComparisonOpKind,
        right: Box<Ast>,
    },

    LogicalExpr {
        left: Box<Ast>,
        operator: LogicalOpKind,
        right: Box<Ast>,
    },

    NotOp(Box<Ast>),

    VariableDeclaration {
        name: String,
        value_type: ValueType,
        expr: Box<Ast>,
    },

    VariableAssignment {
        name: String,
        operator: AssignmentOpKind,
        expr: Box<Ast>,
    },

    FunctionCall {
        name: String,
        argument: Box<Ast>,
    },
}
