use nom::branch::alt;
use nom::bytes::complete::{is_a, tag, take_until};
use nom::character::complete::{alphanumeric0, char, digit1, multispace0, one_of};
use nom::multi::many0;
use nom::sequence::{delimited, tuple};
use nom::IResult;

use crate::ast::{AssignmentOpKind, Ast, ExprOpKind, UnaryOpKind, VariableType};

pub fn parse(input: &str) -> IResult<&str, Ast> {
    alt((parse_statement, parse_function_call, parse_add_sub))(input)
}

fn parse_function_call(input: &str) -> IResult<&str, Ast> {
    let (input, f_name) = take_until("(")(input)?;
    let (input, f_argument) = delimited(char('('), parse_add_sub, char(')'))(input)?;
    Ok((
        input,
        Ast::FunctionCall {
            name: f_name.to_string(),
            argument: Box::new(f_argument),
        },
    ))
}

fn parse_statement(input: &str) -> IResult<&str, Ast> {
    parse_variable_declaration(input)
}

fn parse_variable_declaration(input: &str) -> IResult<&str, Ast> {
    let (input, v_name) = delimited(
        tag("let"),
        delimited(multispace0, parse_variable_name, multispace0),
        char(':'),
    )(input)?;
    let (input, v_type) = parse_variable_type(input)?;
    let (input, v_op) = parse_assignment_operator(input)?;
    let (input, _) = multispace0(input)?;
    let (input, v_expr) = parse_add_sub(input)?;
    Ok((
        input,
        Ast::VariableDeclaration {
            name: v_name.to_string(),
            data_type: v_type,
            operator: v_op,
            expr: Box::new(v_expr),
        },
    ))
}

fn parse_variable_name(input: &str) -> IResult<&str, &str> {
    is_a("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890_")(input)
}

fn parse_variable_type(input: &str) -> IResult<&str, VariableType> {
    let (input, v_type_str) = delimited(multispace0, alphanumeric0, multispace0)(input)?;
    Ok((
        input,
        match v_type_str {
            "int" => VariableType::Int,
            _ => panic!("Unknown VariableDeclaration type"),
        },
    ))
}

fn parse_assignment_operator(input: &str) -> IResult<&str, AssignmentOpKind> {
    let (input, as_op) = is_a("=+-*/")(input)?;
    Ok((
        input,
        match as_op {
            "=" => AssignmentOpKind::AEqual,
            "+=" => AssignmentOpKind::AAdd,
            "-=" => AssignmentOpKind::ASub,
            "*=" => AssignmentOpKind::AMul,
            "/=" => AssignmentOpKind::AMul,
            "**=" => AssignmentOpKind::AExp,
            _ => panic!("Unknown Assignment Operation"),
        },
    ))
}

fn parse_number(input: &str) -> IResult<&str, Ast> {
    let (input, value_str) = digit1(input)?;
    let value = value_str.parse::<isize>().unwrap();
    Ok((input, Ast::Number(value)))
}

fn parse_variable(input: &str) -> IResult<&str, Ast> {
    let (input, v_name) =
        is_a("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890_")(input)?;
    Ok((input, Ast::Variable(v_name.into())))
}

fn parse_par(input: &str) -> IResult<&str, Ast> {
    delimited(
        one_of("("),
        delimited(multispace0, parse_add_sub, multispace0),
        one_of(")"),
    )(input)
}

fn parse_par_num(input: &str) -> IResult<&str, Ast> {
    alt((parse_par, parse_number, parse_variable))(input)
}

fn parse_unary(input: &str) -> IResult<&str, Ast> {
    let (input, unary_op_chars) = many0(one_of("+-"))(input)?;
    let (input, expr) = parse_par_num(input)?;
    Ok((input, parse_monomial(unary_op_chars, expr)))
}

fn parse_exp(input: &str) -> IResult<&str, Ast> {
    let (input, num_expr) = parse_unary(input)?;
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

fn parse_monomial(unary_op_chars: Vec<char>, expr: Ast) -> Ast {
    unary_op_chars
        .into_iter()
        .fold(expr, |expr, unary_op_chars| Ast::Monomial {
            operator: parse_unary_operator(unary_op_chars),
            expr: Box::new(expr),
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
        '+' => UnaryOpKind::UPlus,
        '-' => UnaryOpKind::UMinus,
        _ => panic!("Unknown Operation"),
    }
}
