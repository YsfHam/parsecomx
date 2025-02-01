use std::collections::VecDeque;

use crate::{errors::AndThenError, Parser};

pub struct AndThen<P1, P2> {
    pub(crate) p1: P1,
    pub(crate) p2: P2
}

impl<P1, P2> Parser for AndThen<P1, P2>
where
    P1: Parser,
    P2: Parser<Input = P1::Input>
{
    type Input = P1::Input;
    type Output = (P1::Output, P2::Output);
    type Error = AndThenError<P1::Error, P2::Error>;

    fn parse(&self, input: Self::Input) -> crate::ParserResult<Self::Input, Self::Output, Self::Error> {
        let Self {
            p1: parser1,
            p2: parser2
        } = self;

        parser1
            .parse(input)
            .map_err(|error| 
                AndThenError::FirstFailed(error)
            )
            .and_then(|rest, output| 
                parser2.parse(rest)
                    .map_out(|out2| (output, out2))
                    .map_err(|error| 
                        AndThenError::SecondFailed(error)
                    )
            )
    }
}

pub struct OrElse<P1, P2> {
    pub(crate) p1: P1,
    pub(crate) p2: P2,
}

impl<P1, P2> Parser for OrElse<P1, P2> 
where
    P1: Parser,
    P2: Parser<Input = P1::Input, Output = P1::Output>,
    P2::Error: Into<P1::Error>
{
    type Input = P1::Input;
    type Output = P1::Output;
    type Error = P1::Error;

    fn parse(&self, input: Self::Input) -> crate::ParserResult<Self::Input, Self::Output, Self::Error> {
        self
        .p1
        .parse(input)
        .or_else(|input| self.p2.parse(input))
        .map_err(Into::into)
    }
}

pub struct Map<P, F> {
    pub(crate) p: P,
    pub(crate) mapper: F
}

impl<P, F, R> Parser for Map<P, F> 
where
    P: Parser,
    F: Fn(P::Output) -> R
{
    type Input = P::Input;
    type Output = R;
    type Error = P::Error;

    fn parse(&self, input: Self::Input) -> crate::ParserResult<Self::Input, Self::Output, Self::Error> {
        self.p
            .parse(input)
            .map_out(|out| (self.mapper)(out))
    }
}

pub struct Many<P> {
    pub(crate) p: P
}

impl<P> Parser for Many<P> 
where P: Parser
{
    type Input = P::Input;
    type Output = VecDeque<P::Output>;
    type Error = P::Error;

    fn parse(&self, input: Self::Input) -> crate::ParserResult<Self::Input, Self::Output, Self::Error> {
        let parse_rec = 
        |this: &Self, rest: Self::Input, output: P::Output| {
            this.parse(rest)
                .map_out(|mut many_out| {
                    many_out.push_front(output);
                    many_out
                })
        };
        
        self.p.parse(input)
            .and_then(|rest, output|
                parse_rec(self, rest, output)
            )
            .or_else(|input| 
                crate::ParserResult::success(input, VecDeque::new())
            )
    }
}