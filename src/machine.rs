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
    pub current: Option<isize>,
    pub variables: HashMap<String, Variable>,
}

impl Machine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run(&mut self, expr: Ast) {
        self.current = self.eval(expr);
    }

    pub fn eval(&mut self, expr: Ast) -> Option<isize> {
        match expr {
            Ast::Number(num) => Some(num),

            Ast::Variable(name) => match self.variables.get(&name).unwrap() {
                Variable::Int(n) => Some(*n),
            },

            Ast::Expr {
                left,
                operator: ExprOpKind::EAdd,
                right,
            } => Some(self.eval(*left).unwrap() + self.eval(*right).unwrap()),

            Ast::Expr {
                left,
                operator: ExprOpKind::ESub,
                right,
            } => Some(self.eval(*left).unwrap() - self.eval(*right).unwrap()),

            Ast::Expr {
                left,
                operator: ExprOpKind::EMul,
                right,
            } => Some(self.eval(*left).unwrap() * self.eval(*right).unwrap()),

            Ast::Expr {
                left,
                operator: ExprOpKind::EDiv,
                right,
            } => Some(self.eval(*left).unwrap() / self.eval(*right).unwrap()),

            Ast::Expr {
                left,
                operator: ExprOpKind::EExp,
                right,
            } => Some(
                self.eval(*left)
                    .unwrap()
                    .pow(self.eval(*right).unwrap().try_into().unwrap()),
            ),

            Ast::Monomial {
                operator: UnaryOpKind::UPlus,
                expr,
            } => Some(self.eval(*expr).unwrap()),

            Ast::Monomial {
                operator: UnaryOpKind::UMinus,
                expr,
            } => Some(-self.eval(*expr).unwrap()),

            Ast::VariableDeclaration {
                name,
                data_type,
                expr,
            } => {
                let variable = match data_type {
                    VariableType::Int => Variable::Int(self.eval(*expr).unwrap()),
                };

                self.variables.insert(name, variable);

                None
            }

            Ast::VariableAssignment {
                name,
                operator,
                expr,
            } => {
                let v_expr = self.eval(*expr).unwrap();
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

                None
            }

            Ast::FunctionCall { name, argument } => {
                match name.as_ref() {
                    "print" => builtin_functions::print(self.eval(*argument).unwrap()),
                    _ => panic!("Unknown Function name"),
                };

                None
            }
        }
    }
}
