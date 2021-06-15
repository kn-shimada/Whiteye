#[derive(Debug, PartialEq)]
pub enum OpKind {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, PartialEq)]
pub enum Ast {
    Number(isize),
    //例: 1 + 2
    Expr {
        operator: OpKind,
        left: Box<Ast>,
        
        right: Box<Ast>,
    },
}

pub fn eval(a: Ast) -> isize {
    match a {
        Ast::Expr{operator, left, right} => eval_expr(operator, *left, *right),
        Ast::Number(n) => n,
    }
    
}

pub fn eval_expr(operator: OpKind, left: Ast, right: Ast) -> isize {
    let l = eval(left);
    let r = eval(right);
    match operator {
        OpKind::Add => l + r,
        OpKind::Sub => l - r,
        OpKind::Mul => l * r,
        OpKind::Div => l / r,
    }
}
