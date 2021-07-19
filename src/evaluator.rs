use std::convert::TryInto;

use crate::ast::{Ast, ExprOpKind, UnaryOpKind};

pub fn evaluate(expr: Ast) -> isize {
    match expr {
        Ast::Number(num) => num,

        Ast::Expr {
            left,
            operator: ExprOpKind::EAdd,
            right,
        } => evaluate(*left) + evaluate(*right),

        Ast::Expr {
            left,
            operator: ExprOpKind::ESub,
            right,
        } => evaluate(*left) - evaluate(*right),

        Ast::Expr {
            left,
            operator: ExprOpKind::EMul,
            right,
        } => evaluate(*left) * evaluate(*right),

        Ast::Expr {
            left,
            operator: ExprOpKind::EDiv,
            right,
        } => evaluate(*left) / evaluate(*right),

        Ast::Expr {
            left,
            operator: ExprOpKind::EExp,
            right,
        } => evaluate(*left).pow(evaluate(*right).try_into().unwrap()),

        Ast::Monomial {
            operator: UnaryOpKind::UPuls,
            right,
        } => evaluate(*right) * 1,

        Ast::Monomial {
            operator: UnaryOpKind::UMinus,
            right,
        } => evaluate(*right) * -1,
    }
}
