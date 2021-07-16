use crate::ast::{Ast, OpKind};

pub fn evaluate(expr: Ast) -> isize {
    match expr {
        Ast::Number(num) => num,
        Ast::Expr {
            left,
            operator: OpKind::Add,
            right,
        } => evaluate(*left) + evaluate(*right),
        Ast::Expr {
            left,
            operator: OpKind::Sub,
            right,
        } => evaluate(*left) - evaluate(*right),
        Ast::Expr {
            left,
            operator: OpKind::Mul,
            right,
        } => evaluate(*left) * evaluate(*right),
        Ast::Expr {
            left,
            operator: OpKind::Div,
            right,
        } => evaluate(*left) / evaluate(*right),
        Ast::Expr {
            left,
            operator: OpKind::Exp,
            right,
        } => evaluate(*left).pow(evaluate(*right) as u32),
    }
}
