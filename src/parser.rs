use nom::character::complete::{digit1, one_of};
use nom::error::VerboseError;
use nom::multi::many0;
use nom::sequence::tuple;
use nom::IResult;

use crate::ast::{Ast, OpKind};

fn parse_e(input: &str) -> IResult<&str, Ast, VerboseError<&str>> {
    let (input, num_expr) = parse_number(input)?;
    let (input, exprs) = many0(tuple((one_of("^"), parse_number)))(input)?;
    Ok((input, parse_expr(num_expr, exprs)))
}

fn parse_mul_div(input: &str) -> IResult<&str, Ast, VerboseError<&str>> {
    let (input, num_expr) = parse_e(input)?;
    let (input, exprs) = many0(tuple((one_of("*/"), parse_e)))(input)?;
    Ok((input, parse_expr(num_expr, exprs)))
}

pub fn parse_add_sub(input: &str) -> IResult<&str, Ast, VerboseError<&str>> {
    let (input, num_expr) = parse_mul_div(input)?;
    let (input, exprs) = many0(tuple((one_of("+-"), parse_mul_div)))(input)?;
    Ok((input, parse_expr(num_expr, exprs)))
}

fn parse_expr(num_expr: Ast, exprs: Vec<(char, Ast)>) -> Ast {
    exprs.into_iter().fold(num_expr, |l_expr, exprs| Ast::Expr {
        left: Box::new(l_expr),
        operator: parse_operator(exprs.0),
        right: Box::new(exprs.1),
    })
}

fn parse_operator(op_char: char) -> OpKind {
    match op_char {
        '+' => OpKind::Add,
        '-' => OpKind::Sub,
        '*' => OpKind::Mul,
        '/' => OpKind::Div,
        '^' => OpKind::Exp,
        _ => panic!("Unknown Operation"),
    }
}

fn parse_number(input: &str) -> IResult<&str, Ast, VerboseError<&str>> {
    let (input, value_str) = digit1(input)?;
    let value = value_str.parse::<isize>().unwrap();
    Ok((input, Ast::Number(value)))
}
