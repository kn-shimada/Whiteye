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
