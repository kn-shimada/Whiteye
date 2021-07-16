use nom::branch::alt;
use nom::character::complete::{digit1, one_of};
use nom::error::VerboseError;
use nom::multi::many0;
use nom::sequence::{delimited, tuple};
use nom::IResult;

use crate::ast::{Ast, OpKind};

pub fn parse(input: &str) -> IResult<&str, Ast, VerboseError<&str>> {
    parse_add_sub(input)
}

fn parse_par(input: &str) -> IResult<&str, Ast, VerboseError<&str>> {
    delimited(one_of("("), parse_add_sub, one_of(")"))(input)
}

fn parse_par_num(input: &str) -> IResult<&str, Ast, VerboseError<&str>> {
    alt((parse_par, parse_number))(input)
}

fn parse_exp(input: &str) -> IResult<&str, Ast, VerboseError<&str>> {
    let (input, num_expr) = parse_par_num(input)?;
    let (input, exprs) = many0(tuple((one_of("^"), parse_exp)))(input)?;
    Ok((input, parse_expr(num_expr, exprs)))
}

fn parse_mul_div(input: &str) -> IResult<&str, Ast, VerboseError<&str>> {
    let (input, num_expr) = parse_exp(input)?;
    let (input, exprs) = many0(tuple((one_of("*/"), parse_exp)))(input)?;
    Ok((input, parse_expr(num_expr, exprs)))
}

fn parse_add_sub(input: &str) -> IResult<&str, Ast, VerboseError<&str>> {
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
