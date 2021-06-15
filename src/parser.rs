use anyhow::Result;
use nom::character::complete::{digit1, one_of};
use nom::IResult;
use crate::ast::{OpKind};

pub fn parse(_input: &str) -> Result<()> {
    Ok(())
}

pub fn parse_operator(input: &str) -> IResult<&str, OpKind> {
    let (i, t) = one_of("+-*/")(input)?;
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

pub fn parse_number(input: &str) -> IResult<&str, isize> {
    let (no_used, value_s) = digit1(input)?;
    let value = value_s.parse::<isize>().unwrap();
    Ok((no_used, value))
}
