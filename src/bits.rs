use core::{i128, i16, i32, i64, i8, isize};
use core::{u128, u16, u32, u64, u8, usize};

/// Numbers which have a specified number of bits.
pub trait Bits {
    /// Returns the size of the number in bits.
    fn bits() -> u32;
}

macro_rules! bits_impl {
    ( $($x:ident),* ) => (
        $(
            impl Bits for $x {
                fn bits() -> u32 {
                    $x::BITS
                }
            }
        )*

    );
}

bits_impl!(i128, i16, i32, i64, i8, isize, u128, u16, u32, u64, u8, usize);
