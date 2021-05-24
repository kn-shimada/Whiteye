#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number(isize),

    PrefixExpr{
        operator: String,
        right: Box<Expr>,
    },

    InfixExpr{
        left: Box<Expr>,
        operator: String,
        right: Box<Expr>,
    },
}


