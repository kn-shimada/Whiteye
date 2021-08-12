use std::collections::HashMap;
use std::convert::TryInto;

use crate::ast::{AssignmentOpKind, Ast, ExprOpKind, UnaryOpKind, VariableType};

use crate::builtin_functions;

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

    pub fn run(&mut self, expr: Ast) {
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
            }

            Ast::VariableAssignment {
                name,
                operator,
                expr,
            } => {
                let variable_expr = self.eval_math_expr(*expr);
                let Variable::Int(variable_value) = self.variables.get(&name).unwrap();
                let new_variable_value = match operator {
                    AssignmentOpKind::AEqual => Variable::Int(variable_expr),
                    AssignmentOpKind::AAdd => Variable::Int(variable_value + variable_expr),
                    AssignmentOpKind::ASub => Variable::Int(variable_value - variable_expr),
                    AssignmentOpKind::AMul => Variable::Int(variable_value * variable_expr),
                    AssignmentOpKind::ADiv => Variable::Int(variable_value / variable_expr),
                };
                match self.variables.get_mut(&name) {
                    Some(v) => *v = new_variable_value,
                    None => {
                        panic!("Undefined variable");
                    }
                };
            }

            Ast::FunctionCall { name, argument } => {
                match name.as_ref() {
                    "print" => builtin_functions::print(self.eval_math_expr(*argument)),
                    _ => panic!("Unknown Function name"),
                };
            }

            _ => println!("{}", self.eval_math_expr(expr)),
        };
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

            Ast::Expr {
                left,
                operator: ExprOpKind::EExp,
                right,
            } => self
                .eval_math_expr(*left)
                .pow(self.eval_math_expr(*right).try_into().unwrap()),

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
