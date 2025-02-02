

use crate::{errors::CombinedParsersError, Parser};

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
    type Error = CombinedParsersError<P1::Error, P2::Error>;

    fn parse(&self, input: Self::Input) -> crate::ParserResult<Self::Input, Self::Output, Self::Error> {
        let Self {
            p1: parser1,
            p2: parser2
        } = self;

        let apply_p2 
        = |(rest, output)|
            parser2.parse(rest)
            .map_err(|(input, error)|
                (input, CombinedParsersError::SecondFailed(error))
            )
            .map(|(rest, output2)|
                (rest, (output, output2))
            )
        ;

        let apply_p1 = ||
            parser1
            .parse(input)
            .map_err(|(input, error)| 
                (input, CombinedParsersError::FirstFailed(error))
            )
        ;

        apply_p1().and_then(apply_p2)
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
        .or_else(|(input, _)|self.p2.parse(input))
        .map_err(|(input, error)|
            (input, error.into())
        )
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
            .map(|(rest, output)| 
                (rest, (self.mapper)(output))
            )
    }
}

pub struct Many<P> {
    pub(crate) p: P
}

impl<P> Parser for Many<P> 
where P: Parser
{
    type Input = P::Input;
    type Output = Vec<P::Output>;
    type Error = P::Error;

    fn parse(&self, input: Self::Input) -> crate::ParserResult<Self::Input, Self::Output, Self::Error> {
        let mut result = Vec::new();

        let mut current_input = input;
        loop {
            match self.p.parse(current_input) {
                Ok((rest, output)) => {
                    result.push(output);
                    current_input = rest;
                }
                Err((rest, _)) => {
                    return Ok((
                        rest,
                        result
                    ))
                }
            }
        }
    }
}

pub struct ParseIf<P, Pred, EFn> {
    pub(crate) p: P,
    pub(crate) pred: Pred,
    pub(crate) error: EFn
}

impl<P, Pred, EFn> Parser for ParseIf<P, Pred, EFn> 
where
    P: Parser,
    P::Input: Clone,

    Pred: Fn(&P::Output) -> bool,
    EFn: Fn(&P::Output) -> P::Error
{
    type Input = P::Input;
    type Output = P::Output;
    type Error = P::Error;

    fn parse(&self, input: Self::Input) -> crate::ParserResult<Self::Input, Self::Output, Self::Error> {
        self.p
        .parse(input.clone())
        .and_then(|(rest, output)| {
            if (self.pred)(&output) {
                Ok((rest, output))
            }
            else {
                Err((input, (self.error)(&output)))
            }
        })
    }
}