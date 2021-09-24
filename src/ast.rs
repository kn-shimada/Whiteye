use crate::value::Value;

#[derive(Debug, PartialEq, Clone)]
pub enum ValueType {
    Integer,
    Float,
    Bool,
}

// 演算子
#[derive(Debug, PartialEq, Clone)]
pub enum ExprOpKind {
    EAdd,
    ESub,
    EMul,
    EDiv,
}

// 単項演算子
#[derive(Debug, PartialEq, Clone)]
pub enum UnaryOpKind {
    UPlus,
    UMinus,
}

// 代入演算子
#[derive(Debug, PartialEq, Clone)]
pub enum AssignmentOpKind {
    AEqual,
    AAdd,
    ASub,
    AMul,
    ADiv,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ComparisonOpKind {
    CEqual,
    CNot,
    CGreater,
    CLess,
    CGreaterEqual,
    CLessEqual,
}

#[derive(Debug, PartialEq)]
pub enum LogicalOpKind {
    LAnd,
    LOr,
}

// 抽象構文木
#[derive(Debug, PartialEq, Clone)]
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

    // NotOp(Box<Ast>),
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
