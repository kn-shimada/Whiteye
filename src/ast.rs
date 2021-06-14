#[derive(Debug, Clone, PartialEq)]
pub enum Ast {
    Number(isize),

    //例: 1 + 2
    Expr {
        left: Box<Ast>,
        operator: String,
        right: Box<Expr>,
    },
}


