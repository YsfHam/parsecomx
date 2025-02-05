mod str;
mod byte;

pub use str::*;
pub use byte::*;

use crate::traits::Parser;

pub type ParserResult<I, O, E> = Result<(I, O), (I, E)>;

#[derive(Debug)]
pub enum StringTokenType {
    Int
}

pub(crate) fn parse_many<P: Parser>(parser: &P, input: P::Input) 
    -> ParserResult<P::Input, Vec<P::Output>, P::Error>
{
    let mut result = Vec::new();
    let mut current_input = input;
    loop {
        match parser.parse(current_input) {
            Ok((rest, output)) => {
                result.push(output);

                current_input = rest;
            }
            Err((input, _)) => {
                return Ok((input, result))
            }
        }
    }
}
