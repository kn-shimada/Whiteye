use std::collections::HashMap;
use std::error::Error;

use crate::ast::{AssignmentOpKind, Ast, ExprOpKind, UnaryOpKind, Value, ValueType};

use crate::builtin_functions;

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
                    ValueType::Integer => self.eval_math_expr(*expr),
                    ValueType::Float => self.eval_math_expr(*expr),
                };

                self.variables.insert(name, variable_value);

                Ok(())
            }

            Ast::VariableAssignment {
                name,
                operator,
                expr,
            } => {
                let variable_expr = self.eval_math_expr(*expr);
                if let Value::Integer(variable_value) = match self.variables.get(&name) {
                    Some(v) => v,
                    None => return Err(MachineError::VariableUndefined(name).into()),
                } {
                    let substitute_value = Value::from(*variable_value);
                    let new_variable_value = match operator {
                        AssignmentOpKind::AEqual => variable_expr,
                        AssignmentOpKind::AAdd => variable_expr + substitute_value,
                        AssignmentOpKind::ASub => variable_expr - substitute_value,
                        AssignmentOpKind::AMul => variable_expr * substitute_value,
                        AssignmentOpKind::ADiv => variable_expr / substitute_value,
                    };
                    match self.variables.get_mut(&name) {
                        Some(v) => *v = new_variable_value,
                        None => return Err(MachineError::VariableUndefined(name).into()),
                    };
                }

                Ok(())
            }

            Ast::FunctionCall { name, argument } => {
                match name.as_ref() {
                    "print" => builtin_functions::print(self.eval_math_expr(*argument)),
                    _ => return Err(MachineError::InvalidFunctionName(name).into()),
                };

                Ok(())
            }

            _ => {
                println!("{:?}", self.eval_math_expr(expr));
                Ok(())
            }
        }
    }

    pub fn eval_math_expr(&mut self, expr: Ast) -> Value {
        match expr {
            Ast::Literal(v) => match v {
                Value::Integer(value) => Value::Integer(value),
                Value::Float(value) => Value::Float(value),
            },

            Ast::Variable(name) => match self.variables.get(&name).unwrap() {
                Value::Integer(value) => Value::Integer(*value),
                Value::Float(value) => Value::Float(*value),
            },

            Ast::Expr {
                left,
                operator: ExprOpKind::EAdd,
                right,
            } => self.eval_math_expr(*left) + self.eval_math_expr(*right),

            Ast::Expr {
                left,
                operator: ExprOpKind::ESub,
                right,
            } => self.eval_math_expr(*left) - self.eval_math_expr(*right),

            Ast::Expr {
                left,
                operator: ExprOpKind::EMul,
                right,
            } => self.eval_math_expr(*left) * self.eval_math_expr(*right),

            Ast::Expr {
                left,
                operator: ExprOpKind::EDiv,
                right,
            } => self.eval_math_expr(*left) / self.eval_math_expr(*right),

            Ast::Monomial {
                operator: UnaryOpKind::UPlus,
                expr,
            } => self.eval_math_expr(*expr),

            Ast::Monomial {
                operator: UnaryOpKind::UMinus,
                expr,
            } => Value::from(-1) * self.eval_math_expr(*expr),

            _ => unreachable!(),
        }
    }
}
