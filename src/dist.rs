use std::ops::{Sub, Div, DivAssign};

use {Num};

pub trait Distance {
    type Output: Num;
    fn distance(&self, other: &Self) -> Self::Output;
}

pub trait Norm: Sized  {
    type Output: Num;
    fn norm(&self) -> <Self as Norm>::Output;
}

pub fn normalize<T: Norm<Output=R> + DivAssign<R>, R: Num>(v: &mut T) {
    *v /= v.norm();
}

pub fn normalized<T: Norm<Output=R> + Div<R, Output=T>, R: Num>(v: T) -> T {
    let norm = v.norm();
    v / norm
}

impl<T: Copy + Norm + Sub<Self, Output=Self>> Distance for T{
    type Output = <Self as Norm>::Output;
    fn distance(&self, other: &Self) -> <Self as Distance>::Output {
        (*self - *other).norm()
    }
}


macro_rules! norm_impl_self {
    ($($t:ty)*) => ($(
        impl Norm for $t {
            type Output = Self;
            fn norm(&self) -> <Self as Norm>::Output {
                *self
            }
        }
    )*)
}

macro_rules! norm_impl_abs {
    ($($t:ty)*) => ($(
        impl Norm for $t {
            type Output = Self;
            fn norm(&self) -> <Self as Norm>::Output {
                self.abs()
            }
        }
    )*)
}

macro_rules! norm_impl_unsigned_output {
    ($($t:ty, $out:ty);*) => ($(
        impl Norm for $t {
            type Output = $out;
            fn norm(&self) -> <Self as Norm>::Output {
                self.abs() as $out
            }
        }
    )*)
}

norm_impl_abs!(f32 f64);
norm_impl_unsigned_output!(i8, u8; i16, u16; i32, u32; i64, u64; isize, usize);
norm_impl_self!(u8 u16 u32 u64 usize);

#[cfg(has_i128)]
norm_impl_unsigned_output!(i128, u128);

#[cfg(has_u128)]
norm_impl_self!(u128);