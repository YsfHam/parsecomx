use crate::macros::{floats, signed_integers, unsigned_integers};

unsigned_integers! { u8 u16 u32 u64 u128 usize }
signed_integers! { i8 i16 i32 i64 i128 isize }

floats! { f32 f64 }