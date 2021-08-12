use nom::IResult;

use crate::ast::{Ast};
use super::variable::parse_variable_declaration;

pub fn parse_statement(input: &str) -> IResult<&str, Ast> {
    parse_variable_declaration(input)
}
