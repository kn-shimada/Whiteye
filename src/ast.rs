use std::{convert::From, fmt, ops::{Add, Sub, Mul, Div}};

// 値
#[derive(Debug, PartialEq)]
pub enum Value {
    Integer(isize),
    Float(f64),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Integer(v) => write!(f, "{}", *v),
            Value::Float(v) => write!(f, "{}", *v),
        }
    }
}

impl From<isize> for Value {
    fn from(v: isize) -> Self {
        Value::Integer(v)
    }
}

impl From<f64> for Value {
    fn from(v: f64) -> Self {
        Value::Float(v)
    }
}

impl Add for Value {
    type Output = Value;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Value::Integer(v_lhs) => match rhs {
                Value::Integer(v) => Value::Integer(v_lhs + v),
                Value::Float(v) => Value::Float(v_lhs as f64 + v),
            },
            Value::Float(v_lhs) => match rhs {
                Value::Integer(v) => Value::Float(v_lhs + v as f64),
                Value::Float(v) => Value::Float(v_lhs + v),
            },
        }
    }
}

impl Sub for Value {
    type Output = Value;

    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            Value::Integer(v_lhs) => match rhs {
                Value::Integer(v) => Value::Integer(v_lhs - v),
                Value::Float(v) => Value::Float(v_lhs as f64 - v),
            },
            Value::Float(v_lhs) => match rhs {
                Value::Integer(v) => Value::Float(v_lhs + v as f64),
                Value::Float(v) => Value::Float(v_lhs + v),
            },
        }
    }
}

impl Mul for Value {
    type Output = Value;

    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            Value::Integer(v_lhs) => match rhs {
                Value::Integer(v) => Value::Integer(v_lhs * v),
                Value::Float(v) => Value::Float(v_lhs as f64 * v),
            },
            Value::Float(v_lhs) => match rhs {
                Value::Integer(v) => Value::Float(v_lhs * v as f64),
                Value::Float(v) => Value::Float(v_lhs * v),
            },
        }
    }
}

impl Div for Value {
    type Output = Value;

    fn div(self, rhs: Self) -> Self::Output {
        match self {
            Value::Integer(v_lhs) => match rhs {
                Value::Integer(v) => Value::Integer(v_lhs / v),
                Value::Float(v) => Value::Float(v_lhs as f64 / v),
            },
            Value::Float(v_lhs) => match rhs {
                Value::Integer(v) => Value::Float(v_lhs / v as f64),
                Value::Float(v) => Value::Float(v_lhs / v),
            },
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ValueType {
    Integer,
    Float,
}

// 演算子
#[derive(Debug, PartialEq)]
pub enum ExprOpKind {
    EAdd,
    ESub,
    EMul,
    EDiv,
}

// 単項演算子
#[derive(Debug, PartialEq)]
pub enum UnaryOpKind {
    UPlus,
    UMinus,
}

// 代入演算子
#[derive(Debug, PartialEq)]
pub enum AssignmentOpKind {
    AEqual,
    AAdd,
    ASub,
    AMul,
    ADiv,
}

// 抽象構文木
#[derive(Debug, PartialEq)]
pub enum Ast {
    Literal(Value), // 値

    Variable(String), // 変数

    // 多項式
    Expr {
        left: Box<Ast>,
        operator: ExprOpKind,
        right: Box<Ast>,
    },

    // 単項式
    Monomial {
        operator: UnaryOpKind,
        expr: Box<Ast>,
    },

    // 変数宣言
    VariableDeclaration {
        name: String,
        value_type: ValueType,
        expr: Box<Ast>,
    },

    // 変数への代入
    VariableAssignment {
        name: String,
        operator: AssignmentOpKind,
        expr: Box<Ast>,
    },

    // 組み込み関数の呼び出し
    FunctionCall {
        name: String,
        argument: Box<Ast>,
    },
}
