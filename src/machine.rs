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
                let variable = match data_type {
                    VariableType::Int => Variable::Int(self.eval_math_expr(*expr)),
                };

                self.variables.insert(name, variable);
            }

            Ast::VariableAssignment {
                name,
                operator,
                expr,
            } => {
                let v_expr = self.eval_math_expr(*expr);
                let Variable::Int(v_value) = self.variables.get(&name).unwrap();
                let next_variable = match operator {
                    AssignmentOpKind::AEqual => Variable::Int(v_expr),
                    AssignmentOpKind::AAdd => Variable::Int(v_value + v_expr),
                    AssignmentOpKind::ASub => Variable::Int(v_value - v_expr),
                    AssignmentOpKind::AMul => Variable::Int(v_value * v_expr),
                    AssignmentOpKind::ADiv => Variable::Int(v_value / v_expr),
                };
                match self.variables.get_mut(&name) {
                    Some(v) => *v = next_variable,
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
                Variable::Int(n) => *n,
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
