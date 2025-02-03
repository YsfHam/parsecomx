macro_rules! numbers {
    ($($t:ty)+) => {
        $(
        impl crate::traits::Number for $t {
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


macro_rules! unsigned_numbers {
    ($($t:ty)+) => {
        crate::macros::numbers! {$($t)+}
        crate::macros::mark! {$($t)+, crate::traits::UnsignedNumber}
    };
}

macro_rules! signed_numbers {
    ($($t:ty)+) => {
        crate::macros::numbers! {$($t)+}
        crate::macros::mark! {$($t)+, crate::traits::SignedNumber}
    };
}


pub(crate) use mark;
pub(crate) use signed_numbers;
pub(crate) use numbers;
pub(crate) use unsigned_numbers;