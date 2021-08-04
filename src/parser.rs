use nom::branch::alt;
use nom::bytes::complete::{is_a, tag, take_till};
use nom::character::complete::{digit1, one_of};
use nom::multi::many0;
use nom::sequence::{delimited, tuple};
use nom::IResult;

use crate::ast::{AssignmentOpKind, Ast, ExprOpKind, Statements, UnaryOpKind, VariableType};

pub fn parse(input: &str) -> IResult<&str, Ast> {
    alt((parse_add_sub, parse_statement))(input)
}

fn parse_statement(input: &str) -> IResult<&str, Ast> {
    let (input, statement_str) = take_till(|c| c == ' ')(input)?;
    match statement_str {
        "let" => parse_variable(input),
        _ => panic!("Unknown statemant"),
    }
}

fn parse_variable(input: &str) -> IResult<&str, Ast> {
    let (input, _) = tag(" ")(input)?;
    let (input, v_name) = take_till(|c| c == ':')(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, v_type_str) = take_till(|c| c == ' ')(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, v_operator) = parse_assignment_operator(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, v_expr) = parse_add_sub(input)?;
    Ok((
        input,
        Ast::Variable {
            statement: Statements::Let,
            name: v_name.to_string(),
            data_type: parse_variable_type(v_type_str),
            operator: v_operator,
            expr: Box::new(v_expr),
        }
    ))
}

fn parse_variable_type(v_type_str: &str) -> VariableType {
    match v_type_str {
        "int" => VariableType::Int,
        _ => panic!("Unknown Variable type")
    }
}

fn parse_assignment_operator(input: &str) -> IResult<&str, AssignmentOpKind> {
    let (input, as_op) = is_a("=+-*/")(input)?;
    Ok((
        input,
        match as_op {
            "=" => AssignmentOpKind::AEqual,
            "+=" => AssignmentOpKind::AAdd,
            "-=" => AssignmentOpKind::ESub,
            "*=" => AssignmentOpKind::EMul,
            "/=" => AssignmentOpKind::EMul,
            "**=" => AssignmentOpKind::EExp,
            _ => panic!("Unknown Assignment Operation")
        },
    ))
}

fn parse_number(input: &str) -> IResult<&str, Ast> {
    let (input, value_str) = digit1(input)?;
    let value = value_str.parse::<isize>().unwrap();
    Ok((input, Ast::Number(value)))
}

fn parse_par(input: &str) -> IResult<&str, Ast> {
    delimited(one_of("("), parse_add_sub, one_of(")"))(input)
}

fn parse_par_num(input: &str) -> IResult<&str, Ast> {
    alt((parse_par, parse_number))(input)
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
    unary_op_chars.into_iter().fold(expr, |expr, unary_op_chars| Ast::Monomial {
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
