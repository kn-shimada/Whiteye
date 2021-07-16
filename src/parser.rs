use nom::character::complete::{digit1, one_of};
use nom::error::VerboseError;
use nom::multi::many0;
use nom::sequence::tuple;
use nom::IResult;

use crate::ast::{Ast, OpKind};

pub fn parse(input: &str) -> IResult<&str, Ast, VerboseError<&str>> {
    let (input, num_expr) = parse_number(input)?;
    let (input, exprs) = many0(tuple((parse_operator, parse_number)))(input)?;
    Ok((input, parse_expr(num_expr, exprs)))
}

fn parse_expr(num_expr: Ast, exprs: Vec<(OpKind, Ast)>) -> Ast {
    exprs.into_iter().fold(num_expr, |l_expr, exprs| Ast::Expr {
        left: Box::new(l_expr),
        operator: exprs.0,
        right: Box::new(exprs.1),
    })
}

fn parse_operator(input: &str) -> IResult<&str, OpKind, VerboseError<&str>> {
    let (input, op) = one_of("+-")(input)?;
    Ok((
        input,
        match op {
            '+' => OpKind::Add,
            '-' => OpKind::Sub,
            _ => unreachable!(),
        },
    ))
}

fn parse_number(input: &str) -> IResult<&str, Ast, VerboseError<&str>> {
    let (input, value_str) = digit1(input)?;
    let value = value_str.parse::<isize>().unwrap();
    Ok((input, Ast::Number(value)))
}
