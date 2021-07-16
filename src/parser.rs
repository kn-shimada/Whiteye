use nom::character::complete::{digit1, one_of};
use nom::error::VerboseError;
use nom::multi::many0;
use nom::sequence::tuple;
use nom::IResult;

use crate::ast::{Ast, OpKind};

pub fn parse(input: &str) -> IResult<&str, Ast, VerboseError<&str>> {
    let (input, num1) = parse_number(input)?;
    let (input, exprs) = many0(tuple((parse_operator, parse_number)))(input)?;
    Ok((input, parse_expr(num1, exprs)))
}

fn parse_expr(expr: Ast, rem: Vec<(OpKind, Ast)>) -> Ast {
    rem.into_iter().fold(expr, |acc, val| Ast::Expr {
        left: Box::new(acc),
        operator: val.0,
        right: Box::new(val.1),
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
    let (input, value_s) = digit1(input)?;
    let value = value_s.parse::<isize>().unwrap();
    Ok((input, Ast::Number(value)))
}
