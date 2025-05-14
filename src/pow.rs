use crate::{CheckedMul, One, PrimInt, Unsigned};
use core::num::Wrapping;

/// Binary operator for raising a value to a power.
pub trait Pow<RHS> {
    /// The result after applying the operator.
    type Output;

    /// Returns `self` to the power `rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// use num_traits::Pow;
    /// assert_eq!(Pow::pow(10u32, 2u32), 100);
    /// ```
    fn pow(self, rhs: RHS) -> Self::Output;
}

macro_rules! pow_impl {
    (prim_int $t:ty) => {
        pow_impl!($t, u8);
        pow_impl!($t, u16);
        pow_impl!($t, u32, u32, <$t>::pow);
        pow_impl!($t, u64);
        pow_impl!($t, u128);
        pow_impl!($t, usize);
    };
    ($t:ty) => {
        pow_impl!($t, u8);
        pow_impl!($t, u16);
        pow_impl!($t, u32);
        pow_impl!($t, u64);
        pow_impl!($t, u128);
        pow_impl!($t, usize);
    };
    ($t:ty, $rhs:ty) => {
        pow_impl!($t, $rhs, $rhs, pow);
    };
    ($t:ty, $rhs:ty, $desired_rhs:ty, $method:expr) => {
        impl Pow<$rhs> for $t {
            type Output = $t;
            #[inline]
            fn pow(self, rhs: $rhs) -> $t {
                ($method)(self, <$desired_rhs>::from(rhs))
            }
        }

        impl<'a> Pow<&'a $rhs> for $t {
            type Output = $t;
            #[inline]
            fn pow(self, rhs: &'a $rhs) -> $t {
                ($method)(self, <$desired_rhs>::from(*rhs))
            }
        }

        impl<'a> Pow<$rhs> for &'a $t {
            type Output = $t;
            #[inline]
            fn pow(self, rhs: $rhs) -> $t {
                ($method)(*self, <$desired_rhs>::from(rhs))
            }
        }

        impl<'a, 'b> Pow<&'a $rhs> for &'b $t {
            type Output = $t;
            #[inline]
            fn pow(self, rhs: &'a $rhs) -> $t {
                ($method)(*self, <$desired_rhs>::from(*rhs))
            }
        }
    };
}

pow_impl!(prim_int u8);
pow_impl!(prim_int u16);
pow_impl!(prim_int u32);
pow_impl!(prim_int u64);
pow_impl!(prim_int u128);
pow_impl!(prim_int i8);
pow_impl!(prim_int i16);
pow_impl!(prim_int i32);
pow_impl!(prim_int i64);
pow_impl!(prim_int i128);
pow_impl!(prim_int usize);
pow_impl!(prim_int isize);

pow_impl!(Wrapping<u8>);
pow_impl!(Wrapping<i8>);
pow_impl!(Wrapping<u16>);
pow_impl!(Wrapping<i16>);
pow_impl!(Wrapping<u32>);
pow_impl!(Wrapping<i32>);
pow_impl!(Wrapping<u64>);
pow_impl!(Wrapping<i64>);
pow_impl!(Wrapping<u128>);
pow_impl!(Wrapping<i128>);
pow_impl!(Wrapping<usize>);
pow_impl!(Wrapping<isize>);

#[cfg(any(feature = "std", feature = "libm"))]
mod float_impls {
    use super::Pow;
    use crate::Float;

    pow_impl!(f32, i8, i32, <f32 as Float>::powi);
    pow_impl!(f32, u8, i32, <f32 as Float>::powi);
    pow_impl!(f32, i16, i32, <f32 as Float>::powi);
    pow_impl!(f32, u16, i32, <f32 as Float>::powi);
    pow_impl!(f32, i32, i32, <f32 as Float>::powi);
    pow_impl!(f64, i8, i32, <f64 as Float>::powi);
    pow_impl!(f64, u8, i32, <f64 as Float>::powi);
    pow_impl!(f64, i16, i32, <f64 as Float>::powi);
    pow_impl!(f64, u16, i32, <f64 as Float>::powi);
    pow_impl!(f64, i32, i32, <f64 as Float>::powi);
    pow_impl!(f32, f32, f32, <f32 as Float>::powf);
    pow_impl!(f64, f32, f64, <f64 as Float>::powf);
    pow_impl!(f64, f64, f64, <f64 as Float>::powf);
}

/// Raises a value to the power of exp, using exponentiation by squaring.
///
/// Note that `0⁰` (`pow(0, 0)`) returns `1`. Mathematically this is undefined.
///
/// # Example
///
/// ```rust
/// use num_traits::pow;
///
/// assert_eq!(pow(2i8, 4u32), 16);
/// assert_eq!(pow(6u8, 3u32), 216);
/// assert_eq!(pow(0u8, 0u32), 1); // Be aware if this case affects you
/// ```
#[inline]
pub fn pow<T, U>(mut base: T, mut exp: U) -> T
where
    T: Clone + One,
    U: PrimInt + Unsigned,
{
    if exp == U::zero() {
        return T::one();
    }

    while exp & U::one() == U::zero() {
        base = base.clone() * base;
        exp = exp >> 1;
    }
    if exp == U::one() {
        return base;
    }

    let mut acc = base.clone();
    while exp > U::one() {
        exp = exp >> 1;
        base = base.clone() * base;
        if exp & U::one() == U::one() {
            acc = acc * base.clone();
        }
    }
    acc
}

/// Raises a value to the power of exp, returning `None` if an overflow occurred.
///
/// Note that `0⁰` (`checked_pow(0, 0)`) returns `Some(1)`. Mathematically this is undefined.
///
/// Otherwise same as the `pow` function.
///
/// # Example
///
/// ```rust
/// use num_traits::checked_pow;
///
/// assert_eq!(checked_pow(2i8, 4u32), Some(16));
/// assert_eq!(checked_pow(7i8, 8u32), None);
/// assert_eq!(checked_pow(7u32, 8u32), Some(5_764_801));
/// assert_eq!(checked_pow(0u32, 0u32), Some(1)); // Be aware if this case affect you
/// ```
#[inline]
pub fn checked_pow<T, U>(mut base: T, mut exp: U) -> Option<T>
where
    T: Clone + One + CheckedMul,
    U: PrimInt + Unsigned,
{
    if exp == U::zero() {
        return Some(T::one());
    }

    while exp & U::one() == U::zero() {
        base = base.checked_mul(&base)?;
        exp = exp >> 1;
    }
    if exp == U::one() {
        return Some(base);
    }

    let mut acc = base.clone();
    while exp > U::one() {
        exp = exp >> 1;
        base = base.checked_mul(&base)?;
        if exp & U::one() == U::one() {
            acc = acc.checked_mul(&base)?;
        }
    }
    Some(acc)
}
