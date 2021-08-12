use nom::branch::alt;
use nom::character::complete::{digit1, multispace0, one_of};
use nom::multi::many0;
use nom::sequence::{delimited, tuple};
use nom::IResult;

use super::variable::parse_variable_name;
use crate::ast::{Ast, ExprOpKind, UnaryOpKind};

pub fn parse_add_sub(input: &str) -> IResult<&str, Ast> {
    let (input, left_expr) = parse_mul_div(input)?;
    let (input, exprs) = many0(tuple((one_of("+-"), parse_mul_div)))(input)?;
    Ok((input, parse_expr(left_expr, exprs)))
}

fn parse_mul_div(input: &str) -> IResult<&str, Ast> {
    let (input, left_expr) = parse_exp(input)?;
    let (input, exprs) = many0(tuple((one_of("*/"), parse_exp)))(input)?;
    Ok((input, parse_expr(left_expr, exprs)))
}

fn parse_exp(input: &str) -> IResult<&str, Ast> {
    let (input, left_expr) = parse_unary(input)?;
    let (input, exprs) = many0(tuple((one_of("^"), parse_exp)))(input)?;
    Ok((input, parse_expr(left_expr, exprs)))
}

fn parse_expr(left_expr: Ast, exprs: Vec<(char, Ast)>) -> Ast {
    exprs
        .into_iter()
        .fold(left_expr, |left_expr, exprs| Ast::Expr {
            left: Box::new(left_expr),
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

fn parse_unary(input: &str) -> IResult<&str, Ast> {
    let (input, unary_op_chars) = many0(one_of("+-"))(input)?;
    let (input, expr) = parse_par_num_var(input)?;
    Ok((input, parse_monomial(unary_op_chars, expr)))
}

fn parse_monomial(unary_op_chars: Vec<char>, expr: Ast) -> Ast {
    unary_op_chars
        .into_iter()
        .fold(expr, |expr, unary_op_char| Ast::Monomial {
            operator: parse_unary_operator(unary_op_char),
            expr: Box::new(expr),
        })
}

fn parse_unary_operator(unary_op_char: char) -> UnaryOpKind {
    match unary_op_char {
        '+' => UnaryOpKind::UPlus,
        '-' => UnaryOpKind::UMinus,
        _ => panic!("Unknown Operation"),
    }
}

fn parse_par_num_var(input: &str) -> IResult<&str, Ast> {
    delimited(
        multispace0,
        alt((parse_parentheses, parse_number, parse_variable)),
        multispace0,
    )(input)
}

fn parse_parentheses(input: &str) -> IResult<&str, Ast> {
    delimited(
        one_of("("),
        delimited(multispace0, parse_add_sub, multispace0),
        one_of(")"),
    )(input)
}

fn parse_number(input: &str) -> IResult<&str, Ast> {
    let (input, value_str) = digit1(input)?;
    let value = value_str.parse::<isize>().unwrap();
    Ok((input, Ast::Number(value)))
}

fn parse_variable(input: &str) -> IResult<&str, Ast> {
    let (input, v_name) = parse_variable_name(input)?;
    Ok((input, Ast::Variable(v_name.into())))
}
