use crate::traits::ParserError;

#[derive(Debug)]
pub enum EitherError<E1, E2> {
    LeftError(E1),
    RightError(E2),
}

impl<E1, E2> EitherError<E1, E2> {
    pub fn first_error(self) -> Option<E1> {
        match self {
            EitherError::LeftError(error) => Some(error),
            _ => None,
        }
    }

    pub fn second_error(self) -> Option<E2> {
        match self {
            EitherError::RightError(error) => Some(error),
            _ => None
        }
    } 
}

#[derive(Debug)]
pub enum ParsingErrorKind<E> {
    VerifyError,
    Custom(E),
}

impl<E: ParserError> ParsingErrorKind<E> {
    pub fn into<E2: ParserError>(self) -> ParsingErrorKind<E2>
    where E: Into<E2>
    {
        match self {
            Self::VerifyError => ParsingErrorKind::VerifyError,
            Self::Custom(e) => ParsingErrorKind::Custom(e.into()),
        }
    }

    pub fn map<F, R>(self, f: F) -> ParsingErrorKind<R> 
    where
        F: Fn(E) -> R,
        R: ParserError
    {
        match self {
            ParsingErrorKind::Custom(e) => ParsingErrorKind::Custom(f(e)),
            ParsingErrorKind::VerifyError => ParsingErrorKind::VerifyError
        }
    }
}

impl<E: ParserError> ParserError for ParsingErrorKind<E> {
    fn append(self, new: Self) -> Self {
        match (self, new) {
            (Self::Custom(e), Self::Custom(new_e)) => Self::Custom(e.append(new_e)),
            (Self::Custom(e), _) => Self::Custom(e),
            (_, new) => new
        }
    }
}

impl<E: ParserError> From<E> for ParsingErrorKind<E> {
    fn from(value: E) -> Self {
        Self::Custom(value)
    }
}

#[derive(Debug)]
pub struct ParsingError<E> {
    pub error: ParsingErrorKind<E>,
    pub index: usize
}

impl<E: ParserError> ParsingError<E> {
    pub fn into<E2: ParserError>(self) -> ParsingError<E2>
    where E: Into<E2>
    {
        let Self {
            error,
            index
        } = self;

        ParsingError {
            error: error.into(),
            index
        }
    }
}

impl<E: ParserError> ParserError for ParsingError<E> {
    fn append(self, new: Self) -> Self {
        let Self {
            error,
            index
        } = self;

        Self {
            error: error.append(new.error),
            index
        }
    }
}

#[derive(Debug)]
pub enum StrParsingErrorKind<'a> {
    UnexpectedEnd,
    UnexpectedChar{expected: char, found: char},
    UnexpectedString{expected: &'a str, found: &'a str},
    
    
    ExpectingDigit,
    NumberOverflow,
    InvalidFloat,
}

#[derive(Debug)]
pub struct StrParsingErrors<'a> {
    errors: Vec<StrParsingErrorKind<'a>>
}

impl<'a> StrParsingErrors<'a> {
    pub fn with_error_kind(kind: StrParsingErrorKind<'a>) -> Self {
        Self {
            errors: vec![kind]
        }
    }
}

impl<'a> ParserError for StrParsingErrors<'a> {
    fn append(self, new: Self) -> Self {
        let mut errors = self.errors;
        errors.extend(new.errors);

        Self {errors}
    }
}
