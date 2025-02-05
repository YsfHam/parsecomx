use crate::parsers::StringTokenType;

#[derive(Debug)]
pub enum CombinedParsersError<E1, E2> {
    FirstFailed(E1),
    SecondFailed(E2),
}

impl<E1, E2> CombinedParsersError<E1, E2> {
    pub fn first_error(self) -> Option<E1> {
        match self {
            CombinedParsersError::FirstFailed(error) => Some(error),
            _ => None,
        }
    }

    pub fn second_error(self) -> Option<E2> {
        match self {
            CombinedParsersError::SecondFailed(error) => Some(error),
            _ => None
        }
    } 
}

#[derive(Debug)]
pub enum StringParsingError<'a> {
    UnexpectedEnd,
    UnexpectedChar{expected: char, found: char},
    UnexpectedString{expected: &'a str, found: &'a str},
    UnexpectedCharType{found: char, expected_type: StringTokenType},
    NumberOverflow,
    InvalidFloat,

    IgnoredError,
}

#[derive(Debug)]
pub enum BytesParsingError {
    UnexpectedEnd,
}