use std::num::{ParseFloatError, ParseIntError};
use crate::{combinators::*, errors::{ParsingErrorKind, StrParsingErrors}, parsers::{float_parser, ParserInput, ParserResult}};


pub trait ParserError {

    fn append(self, new: Self) -> Self;
}

pub trait Parser {
    type Input;
    type Output;

    type Error: ParserError;

    fn parse(&self, input: ParserInput<Self::Input>) -> ParserResult<Self::Input, Self::Output, Self::Error>;

    fn and_then<P>(self, other: P) -> AndThen<Self, P> 
    where
        Self: Sized,
        P: Parser
    {
        AndThen { p1: self, p2: other }
    }
    
    fn or_else<P>(self, other: P) -> OrElse<Self, P> 
    where
        Self: Sized,
        P: Parser
    {
        OrElse { p1: self, p2: other }
    }

    fn map<F, R>(self, mapper: F) -> Map<Self, F> 
    where
        Self: Sized,
        F: Fn(Self::Output) -> R
    {
        Map {
            p: self,
            mapper
        }
    }

    fn map_err<F, R>(self, err_mapper: F) -> MapError<Self, F> 
    where
        Self: Sized,
        F: Fn(ParsingErrorKind<Self::Error>) -> R
    {
        MapError {
            p: self,
            err_mapper
        }
    }

    fn map_result<F, O, E>(self, result_mapper: F) -> MapResult<Self, F> 
    where
        Self: Sized,
        F: Fn(Self::Output) -> Result<O, E>
    {
        MapResult {
            p: self,
            result_mapper
        }
    }

    fn many1(self) -> Many1<Self> 
    where Self: Sized,
    {
        Many1 {
            p: self
        }
    }

    fn many(self) -> Many<Self> 
    where Self: Sized,
    {
        Many {
            p: self
        }
    }

    fn then_consume<P>(self, other: P) -> 
    impl Parser<
        Input = Self::Input, 
        Output = Self::Output,
        Error = Self::Error
    > 
    where
        Self: Sized,
        P: Parser<Input = Self::Input, Error = Self::Error>,
    {
        self.and_then(other)
            .map(|(output, _)| output)
    }


    fn then_parse<P>(self, other: P) -> 
    impl Parser<
        Input = Self::Input, 
        Output = P::Output,
        Error = Self::Error
    > 
    where
        Self: Sized,
        P: Parser<Input = Self::Input, Error = Self::Error>,
    {
        self.and_then(other)
            .map(|(_, output)| output)
    }

    fn verify<Pred>(self, pred: Pred) -> Verify<Self, Pred>
    where
        Self: Sized,
        Pred: Fn(&Self::Output) -> bool,
        Self::Input: Clone
    {
        Verify {
            p: self,
            pred
        }
    }

    fn optional(self) -> Optional<Self>
    where Self: Sized
    {
        Optional(self)
    }

    fn flatten(self) -> Flatten<Self>
    where Self: Sized
    {
        Flatten(self)
    }

    fn flat_map<F, P>(self, mapper: F) -> Flatten<Map<Self, F>>
    where
        Self: Sized,
        F: Fn(Self::Output) -> P,
        P: Parser
    {
        self.map(mapper)
        .flatten()
    }

    fn sep_by<SepP>(self, separator: SepP) -> SepBy<Self, SepP>
    where
        Self: Sized,
        SepP: Parser<Input = Self::Input>
    {
        SepBy { p: self, separator }
    }
}

pub trait Number {
    type Inner;
}

pub trait Integer: Number {
    fn from_str(src: &str, radix: u32) -> Result<Self::Inner, ParseIntError>;
}
pub trait Unsigned {}
pub trait Signed {}
mod not_signed {
    pub trait NotSigned {} 
}

impl<T: not_signed::NotSigned> Signed for T {}
impl<T: Unsigned> not_signed::NotSigned for T {}

pub trait ParseableInteger<'a>: Integer {
    fn str_parser(radix: u32) -> impl Parser<Input = &'a str, Output = Self::Inner, Error = StrParsingErrors<'a>>;
}

pub trait Float: Number {
    fn from_str(src: &str) -> Result<Self::Inner, ParseFloatError>;
}

pub trait FloatParser<'a>: Float {
    fn str_parser() -> impl Parser<Input = &'a str, Output = Self::Inner, Error = StrParsingErrors<'a>>;
}

impl<'a, F: Float> FloatParser<'a> for F {
    fn str_parser() -> impl Parser<Input = &'a str, Output = Self::Inner, Error = StrParsingErrors<'a>> {
        float_parser::<Self>()
    }
}