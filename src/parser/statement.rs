use nom::error::VerboseError;
use nom::IResult;

use super::variable::parse_variable_declaration;
use crate::ast::Ast;

pub fn parse_statement(input: &str) -> IResult<&str, Ast, VerboseError<&str>> {
    parse_variable_declaration(input)
}
