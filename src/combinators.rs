

use crate::{errors::CombinedParsersError, collect_many_with_index, Parser};

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

pub struct MapError<P, F> {
    pub(crate) p: P,
    pub(crate) err_mapper: F
}

impl<P, F, R> Parser for MapError<P, F> 
where
    P: Parser,
    F: Fn(P::Error) -> R
{
    type Input = P::Input;
    type Output = P::Output;
    type Error = R;

    fn parse(&self, input: Self::Input) -> crate::ParserResult<Self::Input, Self::Output, Self::Error> {
        self.p
            .parse(input)
            .map_err(|(input, error)| 
                (input, (self.err_mapper)(error))
            )
    }
}


pub struct Many1<P> {
    pub(crate) p: P
}

impl<P> Parser for Many1<P> 
where P: Parser
{
    type Input = P::Input;
    type Output = Vec<P::Output>;
    type Error = P::Error;

    fn parse(&self, input: Self::Input) -> crate::ParserResult<Self::Input, Self::Output, Self::Error> {
        self.p.parse(input)
            .and_then(|(rest, output)|
                collect_many_with_index(&[&self.p], 0, rest)
                .map(|(rest, mut result)|{
                    result.insert(0, output);
                    (rest, result)
                })
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
        collect_many_with_index(&[&self.p], 0, input)
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

pub struct Optional<P> (pub(crate) P);

impl<P> Parser for Optional<P> 
where
    P: Parser
{
    type Input = P::Input;
    type Output = Option<P::Output>;
    type Error = P::Error;

    fn parse(&self, input: Self::Input) -> crate::ParserResult<Self::Input, Self::Output, Self::Error> {
        self.0.parse(input)
        .map_or_else(
            |(input, _)| Ok((input, None)),
            |(rest, output)| Ok((rest, Some(output)))
        )
    }
}

pub struct Flatten<P> (pub(crate) P);

impl<P> Parser for Flatten<P> 
where
    P: Parser,
    P::Output: Parser<Input = P::Input, Error = P::Error>
{
    type Input = P::Input;
    type Output = <P::Output as Parser>::Output;
    type Error = P::Error;

    fn parse(&self, input: Self::Input) -> crate::ParserResult<Self::Input, Self::Output, Self::Error> {
        self.0.parse(input)
        .and_then(|(rest, p)|
            p.parse(rest)
        )
    }
}

pub struct SepBy<P, SepP> {
    pub(crate) p: P,
    pub(crate) separator: SepP
}

impl<P, SepP> Parser for SepBy<P, SepP> 
where
    P: Parser,
    SepP: Parser<Input = P::Input, Output = P::Output, Error = P::Error>
{
    type Input = P::Input;
    type Output = Vec<P::Output>;
    type Error = P::Error;

    fn parse(&self, input: Self::Input) -> crate::ParserResult<Self::Input, Self::Output, Self::Error> {
        self.p
        .parse(input)
        .and_then(|(rest, first)| {
            collect_many_with_index(
                &[&self.separator, &self.p],
                1,
                rest
            )
            .map(|(rest, mut result) | {
                result.push(first);
                (rest, result)
            })
        })
    }
}