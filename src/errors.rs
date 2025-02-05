use crate::parsers::StringTokenType;

#[derive(Debug)]
pub enum CombinedParsersError<E1, E2> {
    FirstFailed(E1),
    SecondFailed(E2)
}

impl<E> CombinedParsersError<E, E> {
    pub fn unwrap_error(self) -> E {
        match self {
            CombinedParsersError::FirstFailed(error) => error,
            CombinedParsersError::SecondFailed(error) => error,
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
    InvalidFloat
}

#[derive(Debug)]
pub enum BytesParsingError {
    UnexpectedEnd,
}