use crate::{errors::{ParsingError, StrParsingErrorKind, StrParsingErrors}, parsers::Parser, traits::{Float, Integer, Signed, Unsigned}};

use super::{ParserInput, ParserResult};


pub fn any_char<'a>() ->
impl Parser<
    Input = &'a str,
    Output = char,
    Error = StrParsingErrors<'a>
>
{
    AnyChar {
        _private: &()
    }
}


pub(crate) fn char_parser<'a>(expected: char) -> 
impl Parser<
    Input = &'a str,
    Output = char,
    Error = StrParsingErrors<'a>
>
{
    any_char()
    .verify(move |c| *c == expected)
}

pub(crate) fn string_parser<'a>(expected: &'a str) -> 
impl Parser<
    Input = &'a str,
    Output = &'a str,
    Error = StrParsingErrors
>
{
    StringParser {
        expected
    }
}

fn number_str_parser<'a>(radix: u32, signed: bool) -> 
impl Parser<
    Input = &'a str,
    Output = String,
    Error = StrParsingErrors<'a>
>
{
    let sign_str = if signed && radix == 10 {"-"} else {""};

    any_char()
    .verify(move |c| c.is_digit(radix))
    .many1()
    .map( move |digits| 
        sign_str.to_string() + &String::from_iter(digits)
    )
    .map_err(|error| {
        match error {
            crate::errors::ParsingErrorKind::VerifyError => 
                StrParsingErrors::with_error_kind(
                    StrParsingErrorKind::ExpectingDigit
                ).into(),
            crate::errors::ParsingErrorKind::Custom(e) => e,
        }
    })

}

pub(crate) fn uint_parser<'a, N: Integer + Unsigned>(radix: u32) -> 
impl Parser<
    Input = &'a str,
    Output = N::Inner,
    Error = StrParsingErrors<'a>
>
{
    number_str_parser(radix,false)
    .map_result(move |number| 
        N::from_str(&number, radix)
        .map_err(|_| StrParsingErrors::with_error_kind(
            StrParsingErrorKind::NumberOverflow
        ).into())
    )
}

pub(crate) fn int_parser<'a, N: Integer + Signed>(radix: u32) -> 
impl Parser<
    Input = &'a str,
    Output = N::Inner,
    Error = StrParsingErrors<'a>
>
{
    '-'
    .optional()
    .flat_map(move |minus| number_str_parser(radix, minus.is_some()))
    .map_result(move |number: String| 
        N::from_str(&number, radix)
        .map_err(|_| StrParsingErrors::with_error_kind(
            StrParsingErrorKind::NumberOverflow
        ).into())
    )
}

pub(crate) fn float_parser<'a, F: Float>() ->
impl Parser<
    Input = &'a str,
    Output = F::Inner,
    Error = StrParsingErrors<'a>
>
{
    let int_part =
        '-' 
        .optional()
        .flat_map(move |minus| number_str_parser(10, minus.is_some()))
        .optional()
    ;

    let decimal_part =
        '.'
        .then_parse(number_str_parser(10, false))
        .optional()
    ;
    int_part.and_then(decimal_part)
    .map_result(|(int_part, decimal_part)|
        F::from_str(&(
            int_part.unwrap_or("".to_string()) + 
            "." + 
            &decimal_part.unwrap_or("".to_string())
        ))
        .map_err(|_| StrParsingErrors::with_error_kind(
            StrParsingErrorKind::InvalidFloat
        ).into())
    )
}

pub fn whitespaces_parser<'a>() -> 
impl Parser<
    Input = &'a str,
    Output = (),
    Error = StrParsingErrors<'a>
>
{
    any_char()
    .verify(|c| c.is_whitespace())
    .many()
    .map(|_| ())
}

pub fn string_literal_parser<'a>() -> 
impl Parser<
    Input = &'a str,
    Output = String,
    Error = StrParsingErrors<'a>
>
{
    // EscapeExpr -> \ then "
    let escape_expr =
        '\\'
        .then_parse(&'"')
        .or_else('n'.map(|_| '\n'))
        .or_else('t'.map(|_| '\t'))
        .or_else('r'.map(|_| '\r'))
    ;

    // character -> any char except " or \ | EscapeExpr
    let character = 
        any_char()
        .verify(|c| *c != '"' && *c != '\\')
        .or_else(escape_expr)
    ;
    // chracters -> many character
    let characters = 
        character
        .many()
        .map(|chars| String::from_iter(chars))
    ;

    // literal -> "chracters"
    '"'
    .then_parse(characters)
    .then_consume(&'"')
    
}

struct AnyChar<'a> {
    _private: &'a ()
}

impl<'a> Parser for AnyChar<'a> {
    type Input = &'a str;
    type Output = char;
    type Error = StrParsingErrors<'a>;

    fn parse(&self, input: ParserInput<Self::Input>) -> ParserResult<Self::Input, Self::Output, Self::Error> {
        let mut chars = input.data.chars();
        let input_index = input.index;
        chars.next()
            .map_or(
                Err((
                    input, 
                    ParsingError {
                        error: StrParsingErrors::with_error_kind(
                            StrParsingErrorKind::UnexpectedEnd
                        ).into(),
                        index: input_index
                    }
                )),
                |c| Ok((
                    ParserInput {
                        data: chars.as_str(),
                        index: input_index + 1
                    },
                    c
                ))
            )
    }
}


struct StringParser<'a> {
    expected: &'a str
}

impl<'a> Parser for StringParser<'a> {
    type Input = &'a str;
    type Output = &'a str;
    type Error = StrParsingErrors<'a>;

    fn parse(&self, input: ParserInput<Self::Input>) -> ParserResult<Self::Input, Self::Output, Self::Error> {
        let data = input.data;
        let index = input.index;
        if data.starts_with(self.expected) {
            Ok((
                ParserInput {
                    data: &data[self.expected.len()..],
                    index: input.index + self.expected.len()
                }, 
                self.expected
            ))
        }
        else {
            Err((
                input,
                ParsingError {
                    error: StrParsingErrors::with_error_kind(StrParsingErrorKind::UnexpectedString{
                        expected: &self.expected,
                        found: data
                    }).into(),
                    index: index
                }
                
            ))
        }
    }
}

impl<'a> Parser for &'a char {
    type Input = &'a str;
    type Output = char;
    type Error = StrParsingErrors<'a>;

    fn parse(&self, input: ParserInput<Self::Input>) -> ParserResult<Self::Input, Self::Output, Self::Error> {
        char_parser(**self).parse(input)
    }
}

impl<'a> Parser for &'a str {
    type Input = &'a str;
    type Output = &'a str;
    type Error = StrParsingErrors<'a>;

    fn parse(&self, input: ParserInput<Self::Input>) -> ParserResult<Self::Input, Self::Output, Self::Error> {
        string_parser(*self).parse(input)
    }
}