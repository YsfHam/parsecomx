
use crate::{errors::ParsingError, Parser};

pub struct AnyChar;

impl Parser for AnyChar {
    type Input = &'static str;
    type Output = char;
    type Error = ParsingError;

    fn parse(&self, input: Self::Input) -> crate::ParserResult<Self::Input, Self::Output, Self::Error> {
        let mut chars = input.chars();
        chars.next()
            .map_or_else(
                || crate::ParserResult::faillure(input, ParsingError::UnexpectedEnd),
                |c| crate::ParserResult::success(chars.as_str(), c)
            )
    }
}

pub fn parse_char(expected: char) -> CharParser {
    CharParser(expected)
}
pub struct CharParser (char);
impl Parser for CharParser {
    type Input = &'static str;
    type Output = char;
    type Error = ParsingError;

    fn parse(&self, input: Self::Input) -> crate::ParserResult<Self::Input, Self::Output, Self::Error> {
        AnyChar.parse(input)
            .and_then(|rest, c| {
                if c == self.0 {
                    crate::ParserResult::success(rest, c)
                }
                else {
                    crate::ParserResult::faillure(input, ParsingError::UnexpectedChar(c))
                }
            })
    }
}