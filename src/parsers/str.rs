use crate::{errors::StringParsingError, parsers::Parser, traits::{Float, SignedInteger, UnsignedInteger}};

use super::{ParserResult, StringTokenType};


pub fn any_char<'a>() ->
impl Parser<
    Input = &'a str,
    Output = char,
    Error = StringParsingError<'a>
>
{
    AnyChar {
        _private: &()
    }
}


pub fn char_parser<'a>(expected: char) -> 
impl Parser<
    Input = &'a str,
    Output = char,
    Error = StringParsingError<'a>
>
{
    any_char()
    .parse_if( move |c| {
        if *c == expected {
            Ok(())
        }
        else {
            Err(
                Some(
                    StringParsingError::UnexpectedChar{expected, found: *c}
                )
            )
        }
    })
    .map_err(|error|
        unsafe {error.unwrap_unchecked()}
    )
}

pub fn string_parser<'a>(expected: &'a str) -> 
impl Parser<
    Input = &'a str,
    Output = &'a str,
    Error = StringParsingError
>
{
    StringParser {
        expected
    }
}

pub fn number_str_parser<'a>(radix: u32, signed: bool) -> 
impl Parser<
    Input = &'a str,
    Output = String,
    Error = StringParsingError<'a>
>
{
    let sign_str = if signed {"-"} else {""};

    any_char()
    .parse_if(move |c|{
        if c.is_digit(radix) {
            Ok(())
        }
        else {
            Err(
                Some(StringParsingError::UnexpectedCharType { 
                    found: *c, 
                    expected_type: StringTokenType::Int
                })
            )
        }
    })
    .map_err(|error|
        unsafe {error.unwrap_unchecked()}
    )
    .many1()
    .map( move |digits| 
        concat_chars(digits, sign_str.to_string())
    )
}

pub fn uint_parser<'a, N: UnsignedInteger>(radix: u32) -> 
impl Parser<
    Input = &'a str,
    Output = N::Inner,
    Error = StringParsingError<'a>
>
{
    number_str_parser(radix,false)
    .map_result(move |number| 
        N::from_str(&number, radix)
        .map_err(|_|StringParsingError::NumberOverflow)
    )
}

pub fn int_parser<'a, N: SignedInteger>(radix: u32) -> 
impl Parser<
    Input = &'a str,
    Output = N::Inner,
    Error = StringParsingError<'a>
>
{
    char_parser('-') // optional
    .optional()
    .flat_map(move |minus| number_str_parser(radix, minus.is_some()))
    .map_result(move |number| 
        N::from_str(&number, radix)
        .map_err(|_|StringParsingError::NumberOverflow)
    )
}

pub fn float_parser<'a, F: Float>() ->
impl Parser<
    Input = &'a str,
    Output = F::Inner,
    Error = StringParsingError<'a>
>
{
    let int_part =
        char_parser('-') 
        .optional()
        .flat_map(move |minus| number_str_parser(10, minus.is_some()))
        .optional()
    ;

    let decimal_part =
        char_parser('.')
        .then_parse_unwrap_error(number_str_parser(10, false))
        .optional()
    ;
    int_part.and_then_unwrap_error(decimal_part)
    .map_result(|(int_part, decimal_part)|
        F::from_str(&(
            int_part.unwrap_or("".to_string()) + 
            "." + 
            &decimal_part.unwrap_or("".to_string())
        ))
        .map_err(|_| StringParsingError::InvalidFloat)
    )
}

pub fn whitespaces_parser<'a>() -> 
impl Parser<
    Input = &'a str,
    Output = (),
    Error = ()
>
{
    any_char()
    .parse_if(|c| {
        if c.is_whitespace() {
            Ok(())
        }
        else {
            Err(None)
        }
    })
    .many()
    .map(|_| ())
}

pub fn string_literal_parser<'a>() -> 
impl Parser<
    Input = &'a str,
    Output = String,
    Error = StringParsingError<'a>
>
{
    // EscapeExpr -> \ then "
    let escape_expr =
        char_parser('\\')
        .then_parse(char_parser('"'))
        .or_else(char_parser('n').map(|_| '\n'))
        .or_else(char_parser('t').map(|_| '\t'))
        .or_else(char_parser('r').map(|_| '\r'))
    ;

    // character -> any char except " or \ | EscapeExpr
    let character = 
        any_char()
        .parse_if(|c|
            if *c != '"' && *c != '\\' {
                Ok(())
            }
            else {
                Err(None)
            }
        )
        .or_else(escape_expr)
    ;
    // chracters -> many character
    let characters = 
        character
        .many()
        .map(|chars| String::from_iter(chars))
    ;

    // literal -> "chracters"
    char_parser('"')
    .then_parse(characters)
    .map_err(|error| unsafe {error.first_error().unwrap_unchecked()})
    .then_consume_unwrap_error(char_parser('"'))
    
}

struct AnyChar<'a> {
    _private: &'a ()
}

impl<'a> Parser for AnyChar<'a> {
    type Input = &'a str;
    type Output = char;
    type Error = StringParsingError<'a>;

    fn parse(&self, input: Self::Input) -> ParserResult<Self::Input, Self::Output, Self::Error> {
        let mut chars = input.chars();
        chars.next()
            .map_or(
                Err((input, StringParsingError::UnexpectedEnd)),
                |c| Ok((chars.as_str(), c))
            )
    }
}


struct StringParser<'a> {
    expected: &'a str
}

impl<'a> Parser for StringParser<'a> {
    type Input = &'a str;
    type Output = &'a str;
    type Error = StringParsingError<'a>;

    fn parse(&self, input: Self::Input) -> ParserResult<Self::Input, Self::Output, Self::Error> {

        if input.starts_with(self.expected) {
            Ok((&input[self.expected.len()..], self.expected))
        }
        else {
            Err((
                input,
                StringParsingError::UnexpectedString{
                    expected: &self.expected,
                    found: &input
                }
            ))
        }
    }
}

fn concat_chars(chars_list: Vec<char>, prefix: String) -> String {
    chars_list
    .iter().fold(prefix, |mut acc, d| {
        acc.push(*d);
        acc
    })
}