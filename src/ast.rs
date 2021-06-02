#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number(isize),

    //例: -10
    PrefixExpr {
        operator: String,
        right: Box<Expr>,
    },

    //例: 1 + 2
    InfixExpr {
        left: Box<Expr>,
        operator: String,
        right: Box<Expr>,
    },
}


