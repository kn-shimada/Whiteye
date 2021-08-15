mod expression;
mod function_call;
mod statement;
mod variable;

use nom::branch::alt;
use nom::error::VerboseError;
use nom::IResult;

use crate::ast::Ast;
use expression::parse_add_sub;
use function_call::parse_function_call;
use statement::parse_statement;
use variable::parse_variable_assignment;

pub fn parse(input: &str) -> IResult<&str, Ast, VerboseError<&str>> {
    alt((
        parse_statement,
        parse_variable_assignment,
        parse_function_call,
        parse_add_sub,
    ))(input)
}
