use combinators::{AndThen, Many, Map, OrElse, ParseIf};
use errors::CombinedParsersError;

pub mod parsers;
pub mod errors;

mod combinators;

pub type ParserResult<I, O, E> = Result<(I, O), (I, E)>;

pub trait Parser: Sized {
    type Input;
    type Output;
    type Error;

    fn parse(&self, input: Self::Input) -> ParserResult<Self::Input, Self::Output, Self::Error>;

    fn and_then<P>(self, other: P) -> AndThen<Self, P> 
    where
        P: Parser
    {
        AndThen { p1: self, p2: other }
    }

    fn or_else<P>(self, other: P) -> OrElse<Self, P> 
    where
        P: Parser
    {
        OrElse { p1: self, p2: other }
    }

    fn map<F, R>(self, mapper: F) -> Map<Self, F> 
    where
        F: Fn(Self::Output) -> R
    {
        Map {
            p: self,
            mapper
        }
    }

    fn many(self) -> Many<Self> {
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
        P: Parser<Input = Self::Input>,
    {
        self.and_then(other)
            .map(|(_, output)| output)
    }

    fn parse_if<Pred, EFn>(self, pred: Pred, error: EFn) -> ParseIf<Self, Pred, EFn>
    where
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
}