use nom::bytes::complete::is_a;
use nom::character::complete::char;
use nom::error::VerboseError;
use nom::sequence::delimited;
use nom::IResult;

use crate::ast::Ast;

use super::expression::parse_add_sub;

pub fn parse_function_call(input: &str) -> IResult<&str, Ast, VerboseError<&str>> {
    let (input, function_name) =
        is_a("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890_")(input)?;
    let (input, function_argument) = delimited(char('('), parse_add_sub, char(')'))(input)?;
    Ok((
        input,
        Ast::FunctionCall {
            name: function_name.to_string(),
            argument: Box::new(function_argument),
        },
    ))
}
