use nom::IResult;

use super::variable::parse_variable_declaration;
use crate::ast::Ast;

pub fn parse_statement(input: &str) -> IResult<&str, Ast> {
    parse_variable_declaration(input)
}
