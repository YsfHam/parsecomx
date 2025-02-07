mod str;

use std::fmt::{Debug, Display};

pub use str::*;

use crate::{errors::ParsingError, traits::Parser};

type GenericResult<I, O, E> = Result<(I, O), (I, E)>;
pub type ParserResult<I, O, E> = GenericResult<ParserInput<I>, O, ParsingError<E>>;


pub struct ParserInput<T> {
    pub(crate) data: T,
    pub(crate) index: usize
}

impl<T: Clone> Clone for ParserInput<T> {
    fn clone(&self) -> Self {
        Self { data: self.data.clone(), index: self.index.clone() }
    }
}

impl<T: Debug> Debug for ParserInput<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ParserInput").field("data", &self.data).field("index", &self.index).finish()
    }
}

impl<T: Display> Display for ParserInput<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data)
    }
}

impl<T> From<T> for ParserInput<T> {
    fn from(value: T) -> Self {
        Self {
            data: value,
            index: 0
        }
    }
}

pub(crate) fn parse_many<P: Parser>(parser: &P, input: ParserInput<P::Input>) 
    -> (ParserInput<P::Input>, Vec<P::Output>)
{
    let mut result = Vec::new();
    let mut current_input = input;
    loop {
        match parser.parse(current_input) {
            Ok((rest, output)) => {
                result.push(output);

                current_input = rest;
            }
            Err((input, _)) => {
                return (input, result)
            }
        }
    }
}
