use crate::{errors::StringParsingError, traits::{SignedNumber, UnsignedNumber}, Parser};


pub fn any_char() ->
impl Parser<
    Input = &'static str,
    Output = char,
    Error = StringParsingError
>
{
    AnyChar
}


pub fn char_parser(expected: char) -> 
impl Parser<
    Input = &'static str,
    Output = char,
    Error = StringParsingError
>
{
    any_char()
    .parse_if( move |c| {
        if *c == expected {
            Ok(())
        }
        else {
            Err(Some(StringParsingError::UnexpectedChar(*c)))
        }
    })
    .map_err(|error|
        unsafe {error.unwrap_unchecked()}
    )
}

pub fn string_parser(expected: &'static str) -> 
impl Parser<
    Input = &'static str,
    Output = &'static str,
    Error = StringParsingError
>
{
    StringParser {
        expected
    }
}

pub fn number_str_parser(radix: u32, signed: bool) -> 
impl Parser<
    Input = &'static str,
    Output = String,
    Error = StringParsingError
>
{
    let sign_str = if signed {"-"} else {""};

    any_char()
    .parse_if(move |c|{
        if c.is_digit(radix) {
            Ok(())
        }
        else {
            Err(Some(StringParsingError::UnexpectedChar(*c)))
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

pub fn uint_parser<N: UnsignedNumber>(radix: u32) -> 
impl Parser<
    Input = &'static str,
    Output = N::Inner,
    Error = StringParsingError
>
{
    number_str_parser(radix,false)
    .map(move |number| N::from_str(&number, radix).unwrap())
}

pub fn int_parser<N: SignedNumber>(radix: u32) -> 
impl Parser<
    Input = &'static str,
    Output = N::Inner,
    Error = StringParsingError
>
{
    char_parser('-') // optional
    .optional()
    .flat_map(move |minus| number_str_parser(radix, minus.is_some()))
    .map(move |number| N::from_str(&number, radix).unwrap())
}

pub fn whitespaces_parser() -> 
impl Parser<
    Input = &'static str,
    Output = String,
    Error = StringParsingError
>
{
    any_char()
    .parse_if(|c| {
        if c.is_whitespace() {
            Ok(())
        }
        else {
            Err(Some(StringParsingError::UnexpectedChar(*c)))
        }
    })
    .map_err(|error|
        unsafe {error.unwrap_unchecked()}
    )
    .many()
    .map(|ws| concat_chars(ws, "".to_string()))
}

struct AnyChar;

impl Parser for AnyChar {
    type Input = &'static str;
    type Output = char;
    type Error = StringParsingError;

    fn parse(&self, input: Self::Input) -> crate::ParserResult<Self::Input, Self::Output, Self::Error> {
        let mut chars = input.chars();
        chars.next()
            .map_or(
                Err((input, StringParsingError::UnexpectedEnd)),
                |c| Ok((chars.as_str(), c))
            )
    }
}


struct StringParser {
    expected: &'static str
}

impl Parser for StringParser {
    type Input = &'static str;
    type Output = &'static str;
    type Error = StringParsingError;

    fn parse(&self, input: Self::Input) -> crate::ParserResult<Self::Input, Self::Output, Self::Error> {
        if input.starts_with(self.expected) {
            Ok((&input[self.expected.len()..], self.expected))
        }
        else {
            Err((input, StringParsingError::UnexpectedString(input)))
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