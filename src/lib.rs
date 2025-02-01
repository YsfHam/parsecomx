use combinators::{AndThen, Many, Map, OrElse};
use errors::CombinedParsersError;

pub mod parsers;
pub mod errors;

mod combinators;

pub struct ParserResult<I, O, E> {
    pub rest: I,
    pub out_result: Result<O, E>,
}

impl<I, O, E> ParserResult<I, O, E> {

    fn new(rest: I, out_result: Result<O, E>) -> Self {
        Self {
            rest,
            out_result
        }
    }

    pub fn success(rest: I, output: O) -> Self {
        Self::new(rest, Ok(output))
    }

    pub fn faillure(rest: I, error: E) -> Self {
        Self::new(rest, Err(error))
    }

    pub fn output(self) -> Option<(I, O)> {
        let ParserResult { rest, out_result } = self;

        out_result.ok()
        .map(|output| (rest, output))
    }

    pub fn map_err<F, R>(self, err_mapper: F) -> ParserResult<I, O, R>
    where
        F: FnOnce(E) -> R
    {
        ParserResult {
            rest: self.rest,
            out_result: 
                self.out_result
                    .map_err(err_mapper)
        }
    }

    pub fn map_out<F, R>(self, out_mapper: F) -> ParserResult<I, R, E>
    where
        F: FnOnce(O) -> R
    {
        ParserResult {
            rest: self.rest,
            out_result: 
                self.out_result
                    .map(out_mapper)
        }
    }

    pub fn and_then<F, R>(self, op: F) -> ParserResult<I, R, E> 
    where
        F: FnOnce(I, O) -> ParserResult<I, R, E>
    {
        let Self {
            rest,
            out_result
        } = self;

        match out_result {
            Ok(output) => op(rest, output),
            Err(e) => ParserResult {
                rest,
                out_result: Err(e)
            }
        }
    }

    pub fn or_else<F, R>(self, op: F) -> ParserResult<I, O, R> 
    where
        F: FnOnce(I) -> ParserResult<I, O, R>
    {
        let Self {
            rest,
            out_result
        } = self;

        match out_result {
            Ok(output) => ParserResult {
                rest,
                out_result: Ok(output)
            },
            Err(_) => op(rest)
        }
    }

}

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
}