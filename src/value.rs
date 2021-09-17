use std::{
    convert::From,
    fmt,
    ops::{Add, Div, Mul, Sub},
};

// 値
#[derive(Debug, PartialEq)]
pub enum Value {
    Integer(isize),
    Float(f64),
    Bool(bool),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Integer(v) => write!(f, "{}", *v),
            Value::Float(v) => write!(f, "{}", *v),
            Value::Bool(v) => write!(f, "{}", *v),
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

impl From<bool> for Value {
    fn from(v: bool) -> Self {
        Value::Bool(v)
    }
}

impl Add for Value {
    type Output = Value;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Value::Integer(v_lhs) => match rhs {
                Value::Integer(v) => Value::Integer(v_lhs + v),
                Value::Float(v) => Value::Float(v_lhs as f64 + v),
                _ => panic!(),
            },

            Value::Float(v_lhs) => match rhs {
                Value::Integer(v) => Value::Float(v_lhs + v as f64),
                Value::Float(v) => Value::Float(v_lhs + v),
                _ => panic!(),
            },

            _ => panic!(),
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
                _ => panic!(),
            },

            Value::Float(v_lhs) => match rhs {
                Value::Integer(v) => Value::Float(v_lhs + v as f64),
                Value::Float(v) => Value::Float(v_lhs + v),
                _ => panic!(),
            },

            _ => panic!(),
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
                _ => panic!(),
            },

            Value::Float(v_lhs) => match rhs {
                Value::Integer(v) => Value::Float(v_lhs * v as f64),
                Value::Float(v) => Value::Float(v_lhs * v),
                _ => panic!(),
            },

            _ => panic!(),
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
                _ => panic!(),
            },

            Value::Float(v_lhs) => match rhs {
                Value::Integer(v) => Value::Float(v_lhs / v as f64),
                Value::Float(v) => Value::Float(v_lhs / v),
                _ => panic!(),
            },

            _ => panic!(),
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self {
            Value::Integer(v_lhs) => match other {
                Value::Integer(v) => v_lhs.partial_cmp(v),
                Value::Float(v) => (*v_lhs as f64).partial_cmp(v),
                _ => panic!(),
            },
            Value::Float(v_lhs) => match other {
                Value::Integer(v) => v_lhs.partial_cmp(&(*v as f64)),
                Value::Float(v) => v_lhs.partial_cmp(v),
                _ => panic!(),
            },
            _ => panic!(),
        }
    }
}
