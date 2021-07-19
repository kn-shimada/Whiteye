use nom::branch::alt;
use nom::character::complete::{digit1, one_of};
use nom::multi::many0;
use nom::sequence::{delimited, tuple};
use nom::IResult;

use crate::ast::{Ast, ExprOpKind, UnaryOpKind};

pub fn parse(input: &str) -> IResult<&str, Ast,> {
    parse_add_sub(input)
}

fn parse_par(input: &str) -> IResult<&str, Ast> {
    delimited(one_of("("), parse_add_sub, one_of(")"))(input)
}

fn parse_par_num(input: &str) -> IResult<&str, Ast> {
    alt((parse_par, parse_number))(input)
}

fn parse_exp(input: &str) -> IResult<&str, Ast> {
    let (input, num_expr) = parse_par_num(input)?;
    let (input, exprs) = many0(tuple((one_of("^"), parse_exp)))(input)?;
    Ok((input, parse_expr(num_expr, exprs)))
}

fn parse_mul_div(input: &str) -> IResult<&str, Ast> {
    let (input, num_expr) = parse_exp(input)?;
    let (input, exprs) = many0(tuple((one_of("*/"), parse_exp)))(input)?;
    Ok((input, parse_expr(num_expr, exprs)))
}

fn parse_add_sub(input: &str) -> IResult<&str, Ast> {
    let (input, num_expr) = parse_mul_div(input)?;
    let (input, exprs) = many0(tuple((one_of("+-"), parse_mul_div)))(input)?;
    Ok((input, parse_expr(num_expr, exprs)))
}

fn parse_expr(num_expr: Ast, exprs: Vec<(char, Ast)>) -> Ast {
    exprs.into_iter().fold(num_expr, |l_expr, exprs| Ast::Expr {
        left: Box::new(l_expr),
        operator: parse_expr_operator(exprs.0),
        right: Box::new(exprs.1),
    })
}

fn parse_expr_operator(expr_op_char: char) -> ExprOpKind {
    match expr_op_char {
        '+' => ExprOpKind::EAdd,
        '-' => ExprOpKind::ESub,
        '*' => ExprOpKind::EMul,
        '/' => ExprOpKind::EDiv,
        '^' => ExprOpKind::EExp,
        _ => panic!("Unknown Operation"),
    }
}

fn parse_unary_operator(unary_op_char: char) -> UnaryOpKind {
    match unary_op_char {
        '+' => UnaryOpKind::UPuls,
        '-' => UnaryOpKind::UMinus
    }
}

fn parse_number(input: &str) -> IResult<&str, Ast> {
    let (input, value_str) = digit1(input)?;
    let value = value_str.parse::<isize>().unwrap();
    Ok((input, Ast::Number(value)))
}
