use core::panic;
use std::collections::HashMap;
use std::convert::TryInto;
use std::error::Error;

use crate::ast::{
    AssignmentOpKind, Ast, ComparisonOpKind, ExprOpKind, LogicalOpKind, UnaryOpKind, ValueType,
};

use crate::{builtin_functions, value::Value};

#[derive(Debug, thiserror::Error)]
pub enum MachineError {
    #[error("Variable Undefined: {0}")]
    VariableUndefined(String),
    #[error("Invalid Function Name: {0}")]
    InvalidFunctionName(String),
}

#[derive(Debug, Default)]
pub struct Machine {
    pub variables: HashMap<String, Value>,
}

impl Machine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run(&mut self, expr: Ast) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        match expr {
            Ast::VariableDeclaration {
                name,
                value_type,
                expr,
            } => {
                let variable_value = match value_type {
                    ValueType::Integer => self.eval_expression(*expr),
                    ValueType::Float => self.eval_expression(*expr),
                    ValueType::Bool => self.eval_expression(*expr),
                };

                self.variables.insert(name, variable_value);

                Ok(())
            }

            Ast::VariableAssignment {
                name,
                operator,
                expr,
            } => {
                let variable_expr = self.eval_expression(*expr);
                let variable_value = match self.variables.get(&name) {
                    Some(v) => match v {
                        Value::Integer(value) => Value::Integer(*value),
                        Value::Float(value) => Value::Float(*value),
                        Value::Bool(value) => Value::Bool(*value),
                    },
                    None => return Err(MachineError::VariableUndefined(name).into()),
                };
                let new_variable_value = match operator {
                    AssignmentOpKind::AEqual => variable_expr,
                    AssignmentOpKind::AAdd => variable_expr + variable_value,
                    AssignmentOpKind::ASub => variable_expr - variable_value,
                    AssignmentOpKind::AMul => variable_expr * variable_value,
                    AssignmentOpKind::ADiv => variable_expr / variable_value,
                };
                match self.variables.get_mut(&name) {
                    Some(v) => *v = new_variable_value,
                    None => return Err(MachineError::VariableUndefined(name).into()),
                };

                Ok(())
            }

            Ast::FunctionCall { name, argument } => {
                match name.as_ref() {
                    "print" => builtin_functions::print(self.eval_expression(*argument)),
                    _ => return Err(MachineError::InvalidFunctionName(name).into()),
                };

                Ok(())
            }

            _ => panic!(),
        }
    }

    pub fn eval_expression(&mut self, expr: Ast) -> Value {
        match expr {
            Ast::Literal(v) => match v {
                Value::Integer(value) => Value::Integer(value),
                Value::Float(value) => Value::Float(value),
                Value::Bool(value) => Value::Bool(value),
            },

            Ast::Variable(name) => match self.variables.get(&name).unwrap() {
                Value::Integer(value) => Value::Integer(*value),
                Value::Float(value) => Value::Float(*value),
                Value::Bool(value) => Value::Bool(*value),
            },

            Ast::Expr {
                left,
                operator: ExprOpKind::EAdd,
                right,
            } => self.eval_expression(*left) + self.eval_expression(*right),

            Ast::Expr {
                left,
                operator: ExprOpKind::ESub,
                right,
            } => self.eval_expression(*left) - self.eval_expression(*right),

            Ast::Expr {
                left,
                operator: ExprOpKind::EMul,
                right,
            } => self.eval_expression(*left) * self.eval_expression(*right),

            Ast::Expr {
                left,
                operator: ExprOpKind::EDiv,
                right,
            } => self.eval_expression(*left) / self.eval_expression(*right),

            Ast::Monomial {
                operator: UnaryOpKind::UPlus,
                expr,
            } => self.eval_expression(*expr),

            Ast::Monomial {
                operator: UnaryOpKind::UMinus,
                expr,
            } => -self.eval_expression(*expr),

            Ast::ComparisonExpr {
                left,
                operator: ComparisonOpKind::CEqual,
                right,
            } => Value::from(self.eval_expression(*left) == self.eval_expression(*right)),

            Ast::ComparisonExpr {
                left,
                operator: ComparisonOpKind::CNot,
                right,
            } => Value::from(self.eval_expression(*left) != self.eval_expression(*right)),

            Ast::ComparisonExpr {
                left,
                operator: ComparisonOpKind::CGreater,
                right,
            } => Value::from(self.eval_expression(*left) > self.eval_expression(*right)),

            Ast::ComparisonExpr {
                left,
                operator: ComparisonOpKind::CLess,
                right,
            } => Value::from(self.eval_expression(*left) < self.eval_expression(*right)),

            Ast::ComparisonExpr {
                left,
                operator: ComparisonOpKind::CGreaterEqual,
                right,
            } => Value::from(self.eval_expression(*left) >= self.eval_expression(*right)),

            Ast::ComparisonExpr {
                left,
                operator: ComparisonOpKind::CLessEqual,
                right,
            } => Value::from(self.eval_expression(*left) <= self.eval_expression(*right)),

            Ast::LogicalExpr {
                left,
                operator: LogicalOpKind::LAnd,
                right,
            } => {
                let left_value = match self.eval_expression(*left).try_into() {
                    Ok(v) => v,
                    Err(_) => panic!(),
                };
                let right_value = match self.eval_expression(*right).try_into() {
                    Ok(v) => v,
                    Err(_) => panic!(),
                };
                Value::from(left_value && right_value)
            }

            Ast::LogicalExpr {
                left,
                operator: LogicalOpKind::LOr,
                right,
            } => {
                let left_value = match self.eval_expression(*left).try_into() {
                    Ok(v) => v,
                    Err(_) => panic!(),
                };
                let right_value = match self.eval_expression(*right).try_into() {
                    Ok(v) => v,
                    Err(_) => panic!(),
                };
                Value::from(left_value || right_value)
            }

            _ => unreachable!(),
        }
    }
}
