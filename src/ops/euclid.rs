use core::ops::{Div, Rem};
use Float;

pub trait DivRemEuclid: Sized + Div<Self, Output = Self> + Rem<Self, Output = Self> {
    /// Calculates Euclidean division, the matching method for `rem_euclid`.
    ///
    /// This computes the integer `n` such that
    /// `self = n * rhs + self.rem_euclid(rhs)`.
    /// In other words, the result is `self / rhs` rounded to the integer `n`
    /// such that `self >= n * rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// use num_traits::DivRemEuclid;
    ///
    /// let a: i32 = 7;
    /// let b: i32 = 4;
    /// assert_eq!(DivRemEuclid::div_euclid(&a,&b), 1); // 7 > 4 * 1
    /// assert_eq!(DivRemEuclid::div_euclid(&-a,&b), -2); // -7 >= 4 * -2
    /// assert_eq!(DivRemEuclid::div_euclid(&a,&-b), -1); // 7 >= -4 * -1
    /// assert_eq!(DivRemEuclid::div_euclid(&-a,&-b), 2); // -7 >= -4 * 2
    /// ```
    fn div_euclid(&self, v: &Self) -> Self;

    /// Calculates the least nonnegative remainder of `self (mod rhs)`.
    ///
    /// In particular, the return value `r` satisfies `0.0 <= r < rhs.abs()` in
    /// most cases. However, due to a floating point round-off error it can
    /// result in `r == rhs.abs()`, violating the mathematical definition, if
    /// `self` is much smaller than `rhs.abs()` in magnitude and `self < 0.0`.
    /// This result is not an element of the function's codomain, but it is the
    /// closest floating point number in the real numbers and thus fulfills the
    /// property `self == self.div_euclid(rhs) * rhs + self.rem_euclid(rhs)`
    /// approximatively.
    ///
    /// # Examples
    ///
    /// ```
    /// use num_traits::DivRemEuclid;
    ///
    /// let a: f32 = 7.0;
    /// let b: f32 = 4.0;
    /// assert_eq!(DivRemEuclid::rem_euclid(&a,&b), 3.0);
    /// assert_eq!(DivRemEuclid::rem_euclid(&-a,&b), 1.0);
    /// assert_eq!(DivRemEuclid::rem_euclid(&a,&-b), 3.0);
    /// assert_eq!(DivRemEuclid::rem_euclid(&-a,&-b), 1.0);
    /// ```
    fn rem_euclid(&self, v: &Self) -> Self;
}
macro_rules! div_rem_euclid_impl {
    ($trait_name:ident for $($t:ty)*) => {$(
        impl $trait_name for $t {
            #[inline]
            fn div_euclid(&self, v: &$t) -> Self {
                <$t>::div_euclid(*self, *v)
            }

            #[inline]
            fn rem_euclid(&self, v: &$t) -> Self {
                <$t>::rem_euclid(*self, *v)
            }

        }
    )*}
}
div_rem_euclid_impl!(DivRemEuclid for isize usize i8 u8 i16 u16 i32 u32 i64 u64);
#[cfg(has_i128)]
div_rem_euclid_impl!(DivRemEuclid for i128 u128);

#[cfg(any(feature = "std", feature = "libm"))]
impl DivRemEuclid for f32 {
    fn div_euclid(&self, rhs: &f32) -> f32 {
        let q = <f32 as Float>::trunc(self / rhs);
        if self % rhs < 0.0 {
            return if *rhs > 0.0 { q - 1.0 } else { q + 1.0 };
        }
        q
    }

    fn rem_euclid(&self, rhs: &f32) -> f32 {
        let r = self % rhs;
        if r < 0.0 {
            r + <f32 as Float>::abs(*rhs)
        } else {
            r
        }
    }
}

#[cfg(any(feature = "std", feature = "libm"))]
impl DivRemEuclid for f64 {
    fn div_euclid(&self, rhs: &f64) -> f64 {
        let q = <f64 as Float>::trunc(self / rhs);
        if self % rhs < 0.0 {
            return if *rhs > 0.0 { q - 1.0 } else { q + 1.0 };
        }
        q
    }

    fn rem_euclid(&self, rhs: &f64) -> f64 {
        let r = self % rhs;
        if r < 0.0 {
            r + <f64 as Float>::abs(*rhs)
        } else {
            r
        }
    }
}

pub trait CheckedDivRemEuclid: Sized + Div<Self, Output = Self> + Rem<Self, Output = Self> {
    /// Performs euclid division that returns `None` instead of panicking on division by zero
    /// and instead of wrapping around on underflow and overflow.
    fn checked_div_euclid(&self, v: &Self) -> Option<Self>;

    /// Finds the euclid remainder of dividing two numbers, checking for underflow, overflow and
    /// division by zero. If any of that happens, `None` is returned.
    fn checked_rem_euclid(&self, v: &Self) -> Option<Self>;
}

macro_rules! checked_div_rem_euclid_impl {
    ($trait_name:ident for $($t:ty)*) => {$(
        impl $trait_name for $t {
            #[inline]
            fn checked_div_euclid(&self, v: &$t) -> Option<$t> {
                <$t>::checked_div_euclid(*self, *v)
            }

            #[inline]
            fn checked_rem_euclid(&self, v: &$t) -> Option<$t> {
                <$t>::checked_rem_euclid(*self, *v)
            }
        }
    )*}
}
checked_div_rem_euclid_impl!(CheckedDivRemEuclid for isize usize i8 u8 i16 u16 i32 u32 i64 u64);
#[cfg(has_i128)]
checked_div_rem_euclid_impl!(CheckedDivRemEuclid for i128 u128);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn euclid_unsigned() {
        macro_rules! test_euclid {
            ($($t:ident)+) => {
                $(
                    {
                        let x: $t = 10;
                        let y: $t = 3;
                        assert_eq!(DivRemEuclid::div_euclid(&x,&y),3);
                        assert_eq!(DivRemEuclid::rem_euclid(&x,&y),1);
                    }
                )+
            };
        }

        test_euclid!(usize u8 u16 u32 u64 isize);
    }

    #[test]
    fn euclid_signed() {
        macro_rules! test_euclid {
            ($($t:ident)+) => {
                $(
                    {
                        let x: $t = 10;
                        let y: $t = -3;
                        assert_eq!(DivRemEuclid::div_euclid(&x,&y),-3);
                        assert_eq!(DivRemEuclid::div_euclid(&-x,&y),4);
                        assert_eq!(DivRemEuclid::rem_euclid(&x,&y),1);
                        assert_eq!(DivRemEuclid::rem_euclid(&-x,&y),2);
                        let x: $t = $t::MIN+1;
                        let y: $t = -1;
                        assert_eq!(DivRemEuclid::div_euclid(&x,&y),$t::MAX);
                    }
                )+
            };
        }

        test_euclid!(i8 i16 i32 i64);
    }

    #[test]
    #[cfg(any(feature = "std", feature = "libm"))]
    fn euclid_float() {
        macro_rules! test_euclid {
            ($($t:ident)+) => {
                $(
                    {
                        let x: $t = 12.1;
                        let y: $t = 3.2;
                        assert!(DivRemEuclid::div_euclid(&x,&y)*y+DivRemEuclid::rem_euclid(&x,&y)-x
                        <=46.4 * $t::EPSILON);
                        assert!(DivRemEuclid::div_euclid(&x,&-y)*-y+DivRemEuclid::rem_euclid(&x,&-y)-x
                        <= 46.4 * $t::EPSILON);
                        assert!(DivRemEuclid::div_euclid(&-x,&y)*y+DivRemEuclid::rem_euclid(&-x,&y)-(-x)
                        <= 46.4 * $t::EPSILON);
                        assert!(DivRemEuclid::div_euclid(&-x,&-y)*-y+DivRemEuclid::rem_euclid(&-x,&-y)-(-x)
                        <= 46.4 * $t::EPSILON);
                    }
                )+
            };
        }

        test_euclid!(f32 f64);
    }

    #[test]
    fn euclid_checked() {
        macro_rules! test_euclid_checked {
            ($($t:ident)+) => {
                $(
                    {
                        assert_eq!(CheckedDivRemEuclid::checked_div_euclid(&$t::MIN,&-1),None);
                        assert_eq!(CheckedDivRemEuclid::checked_rem_euclid(&$t::MIN,&-1),None);
                        assert_eq!(CheckedDivRemEuclid::checked_div_euclid(&1,&0),None);
                        assert_eq!(CheckedDivRemEuclid::checked_rem_euclid(&1,&0),None);
                    }
                )+
            };
        }

        test_euclid_checked!(i8 i16 i32 i64);
    }
}
