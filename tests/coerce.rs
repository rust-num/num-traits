//! Tests of `num_traits::cast`.

#![cfg_attr(not(feature = "std"), no_std)]

use num_traits::Coerced;

use core::{f32, f64};
use core::{i128, i16, i32, i64, i8, isize};
use core::{u128, u16, u32, u64, u8, usize};

macro_rules! test_stuff {
    ($t1:ty, $($t2:ty)*) => {
        $(
            assert_eq!(<$t1 as Coerced<$t2>>::coerce_into(0x5 as $t1), 0x5 as $t2);
        )*
    };
}

macro_rules! testerino {
    ($($ty:ty)*) => {
        $(
            test_stuff!($ty, f32 f64 i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 usize);
        )*
    };
}

#[test]
fn test_coerce() {
    let x: u64 = 0xff_u8.coerce_into();
    assert_eq!(x, 0xff_u64);

    testerino!(f32 f64 i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 usize);
}
