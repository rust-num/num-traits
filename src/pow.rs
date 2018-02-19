use core::ops::Mul;
use {One, CheckedMul};

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
    /// assert_eq!(10.pow(2), 100);
    /// ```
    fn pow(self, rhs: RHS) -> Self::Output;
}

macro_rules! pow_impl {
    ($t:ty, $rhs:ty, $method:ident) => {
        impl Pow<$rhs> for $t {
            type Output = $t;

            #[inline]
            fn pow(self, rhs: $rhs) -> $t {
                <$t>::$method(self, rhs)
            }
        }

        impl<'a> Pow<&'a $rhs> for $t {
            type Output = $t;
            #[inline]
            fn pow(self, rhs: &'a $rhs) -> $t {
                <$t>::$method(self, *rhs)
            }
        }

        impl<'a> Pow<$rhs> for &'a $t {
            type Output = $t;
            #[inline]
            fn pow(self, rhs: $rhs) -> $t {
                <$t>::$method(*self, rhs)
            }
        }

        impl<'a, 'b> Pow<&'a $rhs> for &'b $t {
            type Output = $t;
            #[inline]
            fn pow(self, rhs: &'a $rhs) -> $t {
                <$t>::$method(*self, *rhs)
            }
        }
    }
}

pow_impl!(u8, u32, pow);
pow_impl!(i8, u32, pow);
pow_impl!(u16, u32, pow);
pow_impl!(i16, u32, pow);
pow_impl!(u32, u32, pow);
pow_impl!(i32, u32, pow);
pow_impl!(u64, u32, pow);
pow_impl!(i64, u32, pow);
pow_impl!(usize, u32, pow);
pow_impl!(isize, u32, pow);

#[cfg(feature = "std")]
mod float_impls {
    use super::Pow;

    pow_impl!(f32, i32, powi);
    pow_impl!(f64, i32, powi);
    pow_impl!(f32, f32, powf);
    pow_impl!(f64, f64, powf);
}


/// Raises a value to the power of exp, using exponentiation by squaring.
///
/// # Example
///
/// ```rust
/// use num_traits::pow;
///
/// assert_eq!(pow(2i8, 4), 16);
/// assert_eq!(pow(6u8, 3), 216);
/// ```
#[inline]
pub fn pow<T: Clone + One + Mul<T, Output = T>>(mut base: T, mut exp: usize) -> T {
    if exp == 0 { return T::one() }

    while exp & 1 == 0 {
        base = base.clone() * base;
        exp >>= 1;
    }
    if exp == 1 { return base }

    let mut acc = base.clone();
    while exp > 1 {
        exp >>= 1;
        base = base.clone() * base;
        if exp & 1 == 1 {
            acc = acc * base.clone();
        }
    }
    acc
}

/// Raises a value to the power of exp, returning `None` if an overflow occurred.
///
/// Otherwise same as the `pow` function.
///
/// # Example
///
/// ```rust
/// use num_traits::checked_pow;
///
/// assert_eq!(checked_pow(2i8, 4), Some(16));
/// assert_eq!(checked_pow(7i8, 8), None);
/// assert_eq!(checked_pow(7u32, 8), Some(5_764_801));
/// ```
#[inline]
pub fn checked_pow<T: Clone + One + CheckedMul>(mut base: T, mut exp: usize) -> Option<T> {
    if exp == 0 { return Some(T::one()) }

    macro_rules! optry {
        ( $ expr : expr ) => {
            if let Some(val) = $expr { val } else { return None }
        }
    }

    while exp & 1 == 0 {
        base = optry!(base.checked_mul(&base));
        exp >>= 1;
    }
    if exp == 1 { return Some(base) }

    let mut acc = base.clone();
    while exp > 1 {
        exp >>= 1;
        base = optry!(base.checked_mul(&base));
        if exp & 1 == 1 {
            acc = optry!(acc.checked_mul(&base));
        }
    }
    Some(acc)
}
