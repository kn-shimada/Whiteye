use std::collections::HashMap;
use std::error::Error;

use crate::ast::{AssignmentOpKind, Ast, ExprOpKind, UnaryOpKind, VariableType};

use crate::builtin_functions;

#[derive(Debug, thiserror::Error)]
pub enum MachineError {
    #[error("Variable Undefined: {0}")]
    VariableUndefined(String),
    #[error("Invalid Function Name: {0}")]
    InvalidFunctionName(String),
    #[error("Invalid Expression: {0}")]
    InvalidExpression(String),
}

#[derive(Debug)]
pub enum Variable {
    Int(isize),
}

#[derive(Debug, Default)]
pub struct Machine {
    pub variables: HashMap<String, Variable>,
}

impl Machine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run(&mut self, expr: Ast) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        match expr {
            Ast::VariableDeclaration {
                name,
                data_type,
                expr,
            } => {
                let variable_value = match data_type {
                    VariableType::Int => Variable::Int(self.eval_math_expr(*expr)),
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
                let Variable::Int(variable_value) = match self.variables.get(&name) {
                    Some(v) => v,
                    None => return Err(MachineError::VariableUndefined(name).into()),
                };
                let new_variable_value = match operator {
                    AssignmentOpKind::AEqual => Variable::Int(variable_expr),
                    AssignmentOpKind::AAdd => Variable::Int(variable_value + variable_expr),
                    AssignmentOpKind::ASub => Variable::Int(variable_value - variable_expr),
                    AssignmentOpKind::AMul => Variable::Int(variable_value * variable_expr),
                    AssignmentOpKind::ADiv => Variable::Int(variable_value / variable_expr),
                };
                match self.variables.get_mut(&name) {
                    Some(v) => *v = new_variable_value,
                    None => return Err(MachineError::VariableUndefined(name).into()),
                };

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
                println!("{}", self.eval_math_expr(expr));
                Ok(())
            }
        }
    }

    pub fn eval_math_expr(&mut self, expr: Ast) -> isize {
        match expr {
            Ast::Number(num) => num,

            Ast::Variable(name) => match self.variables.get(&name).unwrap() {
                Variable::Int(value) => *value,
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
            } => -self.eval_math_expr(*expr),

            _ => unreachable!(),
        }
    }
}
