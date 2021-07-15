use nom::character::complete::{digit1, one_of};
use nom::error::VerboseError;
use nom::branch::alt;
use nom::multi::many0;
use nom::IResult;

use crate::ast::{Ast, OpKind};

pub fn parse(input: &str) -> IResult<&str, Vec<Ast>, VerboseError<&str>> {
    let (input, a) = many0(alt((parse_expr, parse_number)))(input)?;
    Ok((input, a))
}

fn parse_expr(input: &str) -> IResult<&str, Ast, VerboseError<&str>> {
    let (input, l) = parse_number(input)?;
    let (input, o) = parse_operator(input)?;
    let (input, r) = alt((parse_expr, parse_number))(input)?;
    Ok((input,
        Ast::Expr{
            left: Box::new(l),
            operator: o,
            right: Box::new(r),
        }))
}

fn parse_operator(input: &str) -> IResult<&str, OpKind, VerboseError<&str>> {
    let (input, op) = one_of("+-*/")(input)?;
    Ok((
        input,
        match op {
            '+' => OpKind::Add,
            '-' => OpKind::Sub,
            '*' => OpKind::Mul,
            '/' => OpKind::Div,
            _ => unreachable!()
        },
    ))
}

fn parse_number(input: &str) -> IResult<&str, Ast, VerboseError<&str>> {
    let (input, value_s) = digit1(input)?;
    let value = value_s.parse::<isize>().unwrap();
    Ok((input, Ast::Number(value)))
}