#[derive(Debug)]
pub enum CombinedParsersError<E1, E2> {
    FirstFailed(E1),
    SecondFailed(E2)
}

#[derive(Debug)]
pub enum ParsingError {
    UnexpectedEnd,
    UnexpectedChar(char)
}