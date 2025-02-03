use crate::{errors::BytesParsingError, Parser};

pub fn parse_bytes<'a>(count: usize) ->
impl Parser<
    Input = &'a [u8],
    Output = &'a [u8],
    Error = BytesParsingError
>
{
    GetBytes {
        bytes_count: count,
        _private: &()
    }
}

struct GetBytes<'a> {
    bytes_count: usize,

    _private: &'a ()
}

impl<'a> Parser for GetBytes<'a> {
    type Input =  &'a [u8];
    type Output = &'a [u8];
    type Error = BytesParsingError;

    fn parse(&self, input: Self::Input) -> crate::ParserResult<Self::Input, Self::Output, Self::Error> {
        if input.len() > self.bytes_count {
            Ok((
                &input[self.bytes_count..],
                &input[0..self.bytes_count]
            ))
        }
        else {
            Err((input, BytesParsingError::UnexpectedEnd))
        }
    }
}