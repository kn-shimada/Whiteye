use nom::character::streaming::space0;
use nom::error::VerboseError;
use nom::IResult;
use nom::{bytes::complete::is_a, sequence::delimited};

use crate::ast::{Ast, ComparisonOpKind};

use super::expression::parse_add_sub;

pub fn parse_conditional_expr(input: &str) -> IResult<&str, Ast, VerboseError<&str>> {
    parse_comparison_expr(input)
}

fn parse_comparison_expr(input: &str) -> IResult<&str, Ast, VerboseError<&str>> {
    let (input, left_expr) = parse_add_sub(input)?;
    let (input, comparison_op) = delimited(space0, parse_comparison_operator, space0)(input)?;
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
