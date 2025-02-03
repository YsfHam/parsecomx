use std::num::ParseIntError;

pub trait Number {
    type Inner;

    fn from_str(src: &str, radix: u32) -> Result<Self::Inner, ParseIntError>;
}

pub trait UnsignedNumber: Number {}
pub trait SignedNumber: Number {}