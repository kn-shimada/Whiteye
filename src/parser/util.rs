use nom::bytes::complete::take_while1;
use nom::error::VerboseError;
use nom::IResult;

pub fn sp(i: &str) -> IResult<&str, &str, VerboseError<&str>> {
    let chars = " \t\r\n";
    take_while1(move |c| chars.contains(c))(i)
}
