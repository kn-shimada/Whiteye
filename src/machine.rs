use std::convert::TryInto;

use crate::ast::{Ast, ExprOpKind, UnaryOpKind};

#[derive(Debug, Default)]
pub struct Machine {
    pub current: isize,
}

impl Machine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run(&mut self, expr: Ast) {
        self.current = self.eval(expr);
    }

    pub fn eval(&self, expr: Ast) -> isize {
        match expr {
            Ast::Number(num) => num,

            Ast::Expr {
                left,
                operator: ExprOpKind::EAdd,
                right,
            } => self.eval(*left) + self.eval(*right),

            Ast::Expr {
                left,
                operator: ExprOpKind::ESub,
                right,
            } => self.eval(*left) - self.eval(*right),

            Ast::Expr {
                left,
                operator: ExprOpKind::EMul,
                right,
            } => self.eval(*left) * self.eval(*right),

            Ast::Expr {
                left,
                operator: ExprOpKind::EDiv,
                right,
            } => self.eval(*left) / self.eval(*right),

            Ast::Expr {
                left,
                operator: ExprOpKind::EExp,
                right,
            } => self.eval(*left).pow(self.eval(*right).try_into().unwrap()),

            Ast::Monomial {
                operator: UnaryOpKind::UPlus,
                expr,
            } => self.eval(*expr),

            Ast::Monomial {
                operator: UnaryOpKind::UMinus,
                expr,
            } => -self.eval(*expr),

            _ => todo!(),
        }
    }
}
