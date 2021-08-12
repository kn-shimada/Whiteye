use nom::bytes::complete::{is_a, tag};
use nom::character::complete::{alphanumeric0, char, multispace0};
use nom::sequence::delimited;
use nom::IResult;

use crate::ast::{Ast, VariableType, AssignmentOpKind};
use super::expression::parse_add_sub;

pub fn parse_variable_assignment(input: &str) -> IResult<&str, Ast> {
    let (input, v_name) = parse_variable_name(input)?;
    let (input, assignment_op) =
        delimited(multispace0, parse_assignment_operator, multispace0)(input)?;
    let (input, v_expr) = parse_add_sub(input)?;
    Ok((
        input,
        Ast::VariableAssignment {
            name: v_name.to_string(),
            operator: assignment_op,
            expr: Box::new(v_expr),
        },
    ))
}

pub fn parse_assignment_operator(input: &str) -> IResult<&str, AssignmentOpKind> {
    let (input, as_op) = is_a("=+-*/")(input)?;
    Ok((
        input,
        match as_op {
            "=" => AssignmentOpKind::AEqual,
            "+=" => AssignmentOpKind::AAdd,
            "-=" => AssignmentOpKind::ASub,
            "*=" => AssignmentOpKind::AMul,
            "/=" => AssignmentOpKind::AMul,
            _ => panic!("Unknown Assignment Operation"),
        },
    ))
}

pub fn parse_variable_declaration(input: &str) -> IResult<&str, Ast> {
    let (input, v_name) = delimited(
        tag("let"),
        delimited(multispace0, parse_variable_name, multispace0),
        char(':'),
    )(input)?;
    let (input, v_type) = parse_variable_type(input)?;
    let (input, _) = char('=')(input)?;
    let (input, _) = multispace0(input)?;
    let (input, v_expr) = parse_add_sub(input)?;
    Ok((
        input,
        Ast::VariableDeclaration {
            name: v_name.to_string(),
            data_type: v_type,
            expr: Box::new(v_expr),
        },
    ))
}

pub fn parse_variable_name(input: &str) -> IResult<&str, &str> {
    is_a("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890_")(input)
}

pub fn parse_variable_type(input: &str) -> IResult<&str, VariableType> {
    let (input, v_type_str) = delimited(multispace0, alphanumeric0, multispace0)(input)?;
    Ok((
        input,
        match v_type_str {
            "int" => VariableType::Int,
            _ => panic!("Unknown VariableDeclaration type"),
        },
    ))
}