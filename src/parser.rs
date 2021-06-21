use nom::character::complete::{digit1, one_of};
use nom::error::VerboseError;
use nom::IResult;

use crate::ast::{Ast, OpKind};

pub fn parse(i: &str) -> IResult<&str, Ast, VerboseError<&str>> {
    let (i, a) = parse_number(i)?;
    while i != "" {
        parse_expr(i, a);
    }
    Ok((i, a))
}

fn parse_expr(i: &str, a: Ast) -> IResult<&str, Ast, VerboseError<&str>> {
    let l;
    if a == (Ast::Expr{isize, OpKind, isize}) | Ast::Number(isize) {
        let i = i;
        l = a; 
    } else {
        let (i, l) = parse_number(i)?;
    }
    let (i, o) = parse_operator(i)?;
    let (i, r) = parse_number(i)?;
    Ok((i,
        Ast::Expr{
            left: Box::new(l),
            operator: o,
            right: Box::new(r),
        }))
}

fn parse_operator(i: &str) -> IResult<&str, OpKind, VerboseError<&str>> {
    let (i, t) = one_of("+-*/")(i)?;
    Ok((
        i,
        match t {
            '+' => OpKind::Add,
            '-' => OpKind::Sub,
            '*' => OpKind::Mul,
            '/' => OpKind::Div,
            _ => unreachable!()
        },
    ))
}

fn parse_number(i: &str) -> IResult<&str, Ast, VerboseError<&str>> {
    let (i, value_s) = digit1(i)?;
    let value = value_s.parse::<isize>().unwrap();
    Ok((i, Ast::Number(value)))
}