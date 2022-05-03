//! Traits for generically calculating distances between values.
//!
//! This is often necessary in generic numeric algorithms to determine if
//! the demanded precision is reached.

use std::ops::{Sub, Div, DivAssign};

use {Num};

/// The abstract notion of the distance between two values.
///
/// This can be used to calculate the distance between two arbitrary
/// values even if there is no sensible definition of a norm of these.
pub trait Distance {
    /// The resulting type of the distance function.
    ///
    /// Mathematically, a norm is a mapping from 2-tuples of vectors of a vector space _V_
    /// into the non-negative real numbers, so `Output` will usually be a floating point type
    /// or in some cases an unsigned integer type.
    type Output: Num;

    /// Calculates the distance between `self` and `other`.
    fn distance(&self, other: &Self) -> Self::Output;
}

/// The abstract notion of the norm of a vector.
///
/// If `Self` is `Copy` and implements `Sub`, then `Distance` will
/// be generically implemented for it. The `distance` function
/// of this generic implementation will calculate the norm of the difference
/// of the two arguments.
pub trait Norm: Sized  {
    /// The resulting type of the norm function.
    ///
    /// Mathematically, a norm is a mapping from a vector space _V_ into the non-negative
    /// real numbers, so `Output` will usually be a floating point type
    /// or in some cases an unsigned integer type.
    type Output: Num;

    /// Calculates the norm of `self`.
    ///
    /// On signed integer and floating point values, it calls the `abs` function.
    ///
    /// On unsigned integer values, it simply returns the original value.
    fn norm(&self) -> <Self as Norm>::Output;
}

/// Normalizes the vector `v`, i.e. divides it by its norm.
///
/// As long as the implementations of `Div` and `DivAssign` on `T` match,
/// `v` will be equal to `normalized(v)` after calling this function.
///
/// ## Attention
///
/// Due to numerical errors, `v` is *not* guaranteed to have exactly norm `1`
/// after calling this function.
///
/// On integer types this function will do complete nonsense since
/// `DivAssign` is implemented as an integer division for integers.
pub fn normalize<T: Norm<Output=R> + DivAssign<R>, R: Num>(v: &mut T) {
    *v /= v.norm();
}

/// Normalizes the normalized vector of `v`, i.e. `v` divided by its norm.
///
/// ## Attention
///
/// Due to numerical errors, the result is *not* guaranteed to have exactly norm `1`
/// after calling this function.
///
/// On integer types this function will do complete nonsense since
/// `Div` is implemented as an integer division for integers.
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


/// Generically implements `Norm` for the unsigned integer types
/// by simply returning the original value.
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

/// Generically implements `Norm` for types with an `abs` function
/// by returning the result of this function.
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

/// Generically implements `Norm` for the signed integer types
/// by calling their `abs` function and casting to the corresponding unsinged
/// integer type.
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


#[test]
fn norm_floating_point() {
    assert_eq!((-2.0f32).norm(), 2.0);
    assert_eq!((-3.0f64).norm(), 3.0);
}

#[test]
fn distance_floating_point() {
    assert_eq!((5.0f32).distance(&3.0), 2.0);
    assert_eq!((2.0f32).distance(&-3.0), 5.0);
    assert_eq!((1.0f64).distance(&4.0), 3.0);
}

#[test]
fn norm_unsigned_integer() {
    assert_eq!(2u8.norm(), 2);
    assert_eq!(3u16.norm(), 3);
    assert_eq!(4u32.norm(), 4);
    assert_eq!(5u64.norm(), 5);
    assert_eq!(6usize.norm(), 6);
}

#[test]
fn norm_signed_integer() {
    assert_eq!((-2i8).norm(), 2);
    assert_eq!((-3i16).norm(), 3);
    assert_eq!((-4i32).norm(), 4);
    assert_eq!((-5i64).norm(), 5);
    assert_eq!((-6isize).norm(), 6);
}