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
pub enum StringParsingError {
    UnexpectedEnd,
    UnexpectedChar(char),
    UnexpectedString(&'static str),
}

#[derive(Debug)]
pub enum BytesParsingError {
    UnexpectedEnd,
}