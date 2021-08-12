mod expression;
mod function_call;
mod statement;
mod variable;

use nom::branch::alt;
use nom::IResult;

use statement::parse_statement;
use variable::parse_variable_assignment;
use function_call::parse_function_call;
use expression::parse_add_sub;
use crate::ast::Ast;

pub fn parse(input: &str) -> IResult<&str, Ast> {
    alt((
        parse_statement,
        parse_variable_assignment,
        parse_function_call,
        parse_add_sub,
    ))(input)
}
