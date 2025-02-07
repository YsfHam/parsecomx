macro_rules! mark {
    ($($t:ty)+, $interface:ty) => {$(
        impl $interface for $t {}
    )+};
}

macro_rules! numbers {
    ($($t:ty)+) => {
        $(
        impl crate::traits::Number for $t {
            type Inner = $t;
        }
    )+};
}

macro_rules! integers {
    ($($t:ty)+) => {

        crate::macros::numbers! {$($t)+}
        $(
        impl crate::traits::Integer for $t {
            fn from_str(src: &str, radix: u32) -> Result<Self::Inner, core::num::ParseIntError> {
                <$t>::from_str_radix(src, radix)
            }
        }
    )+};
}

macro_rules! parseable_integers {
    ($($t:ty)+, $str_parser:expr) => {$(
        impl<'a> crate::traits::ParseableInteger<'a> for $t {
            fn str_parser(radix: u32) -> 
            impl crate::traits::Parser<
                Input = &'a str,
                Output = Self::Inner,
                Error = crate::errors::StrParsingErrors<'a>
            > 
            {
                $str_parser(radix)
            }
        }
    )+};
}

macro_rules! unsigned_integers {
    ($($t:ty)+) => {
        crate::macros::integers! {$($t)+}
        crate::macros::mark! {$($t)+, crate::traits::Unsigned}
        crate::macros::parseable_integers! {
            $($t)+,
            crate::parsers::uint_parser::<Self>
        }
    };
}

macro_rules! signed_integers {
    ($($t:ty)+) => {
        crate::macros::integers! {$($t)+}
        crate::macros::mark! {$($t)+, crate::traits::Signed}
        crate::macros::parseable_integers! {
            $($t)+,
            crate::parsers::int_parser::<Self>
        }
    };
}

macro_rules! floats {
    ($($t:ty)+) => {
        crate::macros::numbers! {$($t)+}
        $(
        impl crate::traits::Float for $t {
            fn from_str(src: &str) -> Result<Self::Inner, core::num::ParseFloatError> {
                src.parse()
            }
        }
    )+};
}


pub(crate) use parseable_integers;
pub(crate) use mark;
pub(crate) use numbers;
pub(crate) use signed_integers;
pub(crate) use integers;
pub(crate) use unsigned_integers;
pub(crate) use floats;