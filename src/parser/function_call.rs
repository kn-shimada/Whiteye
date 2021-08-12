use nom::bytes::complete::is_a;
use nom::character::complete::char;
use nom::sequence::delimited;
use nom::IResult;

use crate::ast::Ast;

use super::expression::parse_add_sub;

pub fn parse_function_call(input: &str) -> IResult<&str, Ast> {
    let (input, f_name) =
        is_a("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890_")(input)?;
    let (input, f_argument) = delimited(char('('), parse_add_sub, char(')'))(input)?;
    Ok((
        input,
        Ast::FunctionCall {
            name: f_name.to_string(),
            argument: Box::new(f_argument),
        },
    ))
}
