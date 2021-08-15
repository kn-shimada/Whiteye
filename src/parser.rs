mod expression;
mod function_call;
mod statement;
mod util;
mod variable;

use nom::branch::alt;
use nom::combinator::opt;
use nom::error::convert_error;
use nom::error::VerboseError;
use nom::sequence::delimited;
use nom::Finish;
use nom::IResult;

use crate::ast::Ast;
use expression::parse_add_sub;
use function_call::parse_function_call;
use statement::parse_statement;
use util::sp;
use variable::parse_variable_assignment;

pub fn parse(input: &str) -> Result<Vec<Ast>, String> {
    let mut result: Vec<Ast> = Vec::new();

    let mut looping = true;

    let mut input = input;
    while looping {
        match root_parser(input).finish() {
            Ok((remain, parsed)) => {
                result.push(parsed);
                input = remain;
                looping = !remain.is_empty();
            }
            Err(e) => return Err(convert_error(input, e)),
        };
    }
    Ok(result)
}

pub fn root_parser(input: &str) -> IResult<&str, Ast, VerboseError<&str>> {
    delimited(
        opt(sp),
        alt((
            parse_statement,
            parse_variable_assignment,
            parse_function_call,
            parse_add_sub,
        )),
        opt(sp),
    )(input)
}
