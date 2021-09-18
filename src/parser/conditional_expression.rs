use nom::bytes::complete::is_a;
use nom::error::VerboseError;
use nom::multi::many0;
use nom::sequence::tuple;
use nom::IResult;

use crate::ast::{Ast, ComparisonOpKind, LogicalOpKind};

use super::expression::parse_add_sub;

pub fn parse_conditional_expr(input: &str) -> IResult<&str, Ast, VerboseError<&str>> {
    parse_and_or(input)
}

fn parse_and_or(input: &str) -> IResult<&str, Ast, VerboseError<&str>> {
    let (input, left_expr) = parse_comparison_expr(input)?;
    let (input, exprs) = many0(tuple((is_a("&|"), parse_comparison_expr)))(input)?;
    Ok((input, parse_logical_expr(left_expr, exprs)))
}

fn parse_logical_expr(left_expr: Ast, exprs: Vec<(&str, Ast)>) -> Ast {
    exprs
        .into_iter()
        .fold(left_expr, |left_expr, exprs| Ast::LogicalExpr {
            left: Box::new(left_expr),
            operator: parse_logical_oprator(exprs.0),
            right: Box::new(exprs.1),
        })
}

fn parse_logical_oprator(input: &str) -> LogicalOpKind {
    match input {
        "&&" => LogicalOpKind::LAnd,
        "||" => LogicalOpKind::LOr,
        _ => panic!("Unknown Operation"),
    }
}

fn parse_comparison_expr(input: &str) -> IResult<&str, Ast, VerboseError<&str>> {
    let (input, left_expr) = parse_add_sub(input)?;
    let (input, comparison_op) = parse_comparison_operator(input)?;
    let (input, right_expr) = parse_add_sub(input)?;
    Ok((
        input,
        Ast::ComparisonExpr {
            left: Box::new(left_expr),
            operator: comparison_op,
            right: Box::new(right_expr),
        },
    ))
}

fn parse_comparison_operator(input: &str) -> IResult<&str, ComparisonOpKind, VerboseError<&str>> {
    let (input, comparison_op_str) = is_a("=><!")(input)?;
    let comparison_op = match comparison_op_str {
        "==" => ComparisonOpKind::CEqual,
        "!=" => ComparisonOpKind::CNot,
        ">" => ComparisonOpKind::CGreater,
        "<" => ComparisonOpKind::CLess,
        ">=" => ComparisonOpKind::CGreaterEqual,
        "<=" => ComparisonOpKind::CLessEqual,
        _ => panic!("Unknown Operation"),
    };
    Ok((input, comparison_op))
}
