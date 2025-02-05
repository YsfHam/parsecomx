use std::num::ParseIntError;
use crate::{combinators::*, errors::CombinedParsersError, parsers::ParserResult};

pub trait Parser {
    type Input;
    type Output;
    type Error;

    fn parse(&self, input: Self::Input) -> ParserResult<Self::Input, Self::Output, Self::Error>;

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
        F: Fn(Self::Error) -> R
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
        Error = 
            CombinedParsersError<
                Self::Error,
                P::Error
            >
    > 
    where
        Self: Sized,
        P: Parser<Input = Self::Input>,
    {
        self.and_then(other)
            .map(|(output, _)| output)
    }


    fn then_parse<P>(self, other: P) -> 
    impl Parser<
        Input = Self::Input, 
        Output = P::Output,
        Error = 
            CombinedParsersError<
                Self::Error,
                P::Error
            >
    > 
    where
        Self: Sized,
        P: Parser<Input = Self::Input>,
    {
        self.and_then(other)
            .map(|(_, output)| output)
    }

    fn parse_if<Pred>(self, pred: Pred) -> ParseIf<Self, Pred>
    where
        Self: Sized,
        Pred: Fn(&Self::Output) -> Result<(), Option<Self::Error>>,
        Self::Input: Clone
    {
        ParseIf {
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
        SepP: Parser<Input = Self::Input, Error = Self::Error>
    {
        SepBy { p: self, separator }
    }

    fn then_consume_optional<P>(self, optional_p: Optional<P>) ->
    impl Parser<
        Input = Self::Input, 
        Output = Self::Output,
        Error = Self::Error
    > 
    where
        Self: Sized,
        P: Parser<Input = Self::Input>
    {
        self.then_consume(optional_p)
        .map_err(|error|
            match error {
                CombinedParsersError::FirstFailed(error) => error,
                CombinedParsersError::SecondFailed(_) => unreachable!(),
            }
        )
    }
}

pub trait Number {
    type Inner;

    fn from_str(src: &str, radix: u32) -> Result<Self::Inner, ParseIntError>;
}

pub trait UnsignedNumber: Number {}
pub trait SignedNumber: Number {}