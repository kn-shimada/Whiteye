use std::collections::HashMap;
use std::convert::TryInto;

use crate::ast::{AssignmentOpKind, Ast, ExprOpKind, UnaryOpKind, VariableType};

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

            Ast::Variable {
                name,
                data_type,
                operator,
                expr,
            } => {
                let variable = match data_type {
                    VariableType::Int => Variable::Int(self.eval(*expr).unwrap()),
                };

                match operator {
                    AssignmentOpKind::AEqual => self.variables.insert(name, variable),
                    _ => todo!(),
                };

                None
            }
        }
    }
}
