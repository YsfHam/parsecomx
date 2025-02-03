use std::ops::Add;

use combinators::{AndThen, Flatten, Many, Many1, Map, MapError, Optional, OrElse, ParseIf, SepBy};
use errors::CombinedParsersError;

pub mod parsers;
pub mod errors;
pub mod traits;

mod macros;
mod combinators;
mod defs;

pub type ParserResult<I, O, E> = Result<(I, O), (I, E)>;

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

    fn parse_if<Pred, EFn>(self, pred: Pred, error: EFn) -> ParseIf<Self, Pred, EFn>
    where
        Self: Sized,
        Pred: Fn(&Self::Output) -> bool,
        EFn: Fn(&Self::Output) -> Self::Error,
        Self::Input: Clone
    {
        ParseIf {
            p: self,
            pred,
            error
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
        SepP: Parser<Input = Self::Input, Output = Self::Output, Error = Self::Error>
    {
        SepBy { p: self, separator }
    }
}

pub(crate) fn collect_many_with_index<Input, Output, Error>
(
    parsers: &[&dyn Parser<Input = Input, Output = Output, Error = Error>],
    index: usize,
    input: Input
) -> 
ParserResult<Input, Vec<Output>, Error>
{
    let mut result = Vec::new();
    let mut current_input = input;
    let mut current_parser_index = 0;
    loop {
        match parsers[current_parser_index].parse(current_input) {
            Ok((rest, output)) => {
                if current_parser_index == index {
                    result.push(output);
                }

                current_input = rest;
                current_parser_index = 
                    current_parser_index.add(1).clamp(0, parsers.len() - 1);
            }
            Err((input, _)) => {
                return Ok((input, result))
            }
        }
    }
}