
use crate::{errors::ParsingError, Parser};


pub fn any_char() ->
impl Parser<
    Input = &'static str,
    Output = char,
    Error = ParsingError
>
{
    AnyChar
}


pub fn parse_char(expected: char) -> 
impl Parser<
    Input = &'static str,
    Output = char,
    Error = ParsingError
>
{
    any_char()
    .parse_if( 
        move |c| *c == expected, 
        |c| ParsingError::UnexpectedChar(*c))
}

struct AnyChar;

impl Parser for AnyChar {
    type Input = &'static str;
    type Output = char;
    type Error = ParsingError;

    fn parse(&self, input: Self::Input) -> crate::ParserResult<Self::Input, Self::Output, Self::Error> {
        let mut chars = input.chars();
        chars.next()
            .map_or(
                Err((input, ParsingError::UnexpectedEnd)),
                |c| Ok((chars.as_str(), c))
            )
    }
}