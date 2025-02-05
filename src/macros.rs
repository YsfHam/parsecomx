macro_rules! integers {
    ($($t:ty)+) => {
        $(
        impl crate::traits::Integer for $t {
            type Inner = $t;
            fn from_str(src: &str, radix: u32) -> Result<Self::Inner, core::num::ParseIntError> {
                <$t>::from_str_radix(src, radix)
            }
        }
    )+};
}

macro_rules! mark {
    ($($t:ty)+, $interface:ty) => {$(
        impl $interface for $t {}
    )+};
}


macro_rules! unsigned_integers {
    ($($t:ty)+) => {
        crate::macros::integers! {$($t)+}
        crate::macros::mark! {$($t)+, crate::traits::UnsignedInteger}
    };
}

macro_rules! signed_integers {
    ($($t:ty)+) => {
        crate::macros::integers! {$($t)+}
        crate::macros::mark! {$($t)+, crate::traits::SignedInteger}
    };
}

macro_rules! floats {
    ($($t:ty)+) => {
        $(
        impl crate::traits::Float for $t {
            type Inner = $t;
            fn from_str(src: &str) -> Result<Self::Inner, core::num::ParseFloatError> {
                src.parse()
            }
        }
    )+};
}


pub(crate) use mark;
pub(crate) use signed_integers;
pub(crate) use integers;
pub(crate) use unsigned_integers;
pub(crate) use floats;