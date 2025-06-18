use core::ops::{Div, Rem};

pub trait Euclid: Sized + Div<Self, Output = Self> + Rem<Self, Output = Self> {
    /// Calculates Euclidean division, the matching method for `rem_euclid`.
    ///
    /// This computes the integer `n` such that
    /// `self = n * v + self.rem_euclid(v)`.
    /// In other words, the result is `self / v` rounded to the integer `n`
    /// such that `self >= n * v`.
    ///
    /// # Examples
    ///
    /// ```
    /// use num_traits::Euclid;
    ///
    /// let a: i32 = 7;
    /// let b: i32 = 4;
    /// assert_eq!(Euclid::div_euclid(&a, &b), 1); // 7 > 4 * 1
    /// assert_eq!(Euclid::div_euclid(&-a, &b), -2); // -7 >= 4 * -2
    /// assert_eq!(Euclid::div_euclid(&a, &-b), -1); // 7 >= -4 * -1
    /// assert_eq!(Euclid::div_euclid(&-a, &-b), 2); // -7 >= -4 * 2
    /// ```
    fn div_euclid(&self, v: &Self) -> Self;

    /// Calculates the least nonnegative remainder of `self (mod v)`.
    ///
    /// In particular, the return value `r` satisfies `0.0 <= r < v.abs()` in
    /// most cases. However, due to a floating point round-off error it can
    /// result in `r == v.abs()`, violating the mathematical definition, if
    /// `self` is much smaller than `v.abs()` in magnitude and `self < 0.0`.
    /// This result is not an element of the function's codomain, but it is the
    /// closest floating point number in the real numbers and thus fulfills the
    /// property `self == self.div_euclid(v) * v + self.rem_euclid(v)`
    /// approximatively.
    ///
    /// # Examples
    ///
    /// ```
    /// use num_traits::Euclid;
    ///
    /// let a: i32 = 7;
    /// let b: i32 = 4;
    /// assert_eq!(Euclid::rem_euclid(&a, &b), 3);
    /// assert_eq!(Euclid::rem_euclid(&-a, &b), 1);
    /// assert_eq!(Euclid::rem_euclid(&a, &-b), 3);
    /// assert_eq!(Euclid::rem_euclid(&-a, &-b), 1);
    /// ```
    fn rem_euclid(&self, v: &Self) -> Self;

    /// Returns both the quotient and remainder from Euclidean division.
    ///
    /// By default, it internally calls both `Euclid::div_euclid` and `Euclid::rem_euclid`,
    /// but it can be overridden in order to implement some optimization.
    ///
    /// # Examples
    ///
    /// ```
    /// # use num_traits::Euclid;
    /// let x = 5u8;
    /// let y = 3u8;
    ///
    /// let div = Euclid::div_euclid(&x, &y);
    /// let rem = Euclid::rem_euclid(&x, &y);
    ///
    /// assert_eq!((div, rem), Euclid::div_rem_euclid(&x, &y));
    /// ```
    fn div_rem_euclid(&self, v: &Self) -> (Self, Self) {
        (self.div_euclid(v), self.rem_euclid(v))
    }
}

macro_rules! euclid_forward_impl {
    ($($t:ty)*) => {$(
        impl Euclid for $t {
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

euclid_forward_impl!(isize i8 i16 i32 i64 i128);
euclid_forward_impl!(usize u8 u16 u32 u64 u128);

#[cfg(has_f16)]
#[cfg(feature = "std")]
euclid_forward_impl!(f16);

#[cfg(feature = "std")]
euclid_forward_impl!(f32 f64);

#[cfg(has_f128)]
#[cfg(feature = "std")]
euclid_forward_impl!(f128);

#[cfg(has_f16)]
#[cfg(not(feature = "std"))]
impl Euclid for f16 {
    #[inline]
    fn div_euclid(&self, v: &f16) -> f16 {
        let q = <f16 as crate::float::FloatCore>::trunc(self / v);
        if self % v < 0.0 {
            return if *v > 0.0 { q - 1.0 } else { q + 1.0 };
        }
        q
    }

    #[inline]
    fn rem_euclid(&self, v: &f16) -> f16 {
        let r = self % v;
        if r < 0.0 {
            r + <f16 as crate::float::FloatCore>::abs(*v)
        } else {
            r
        }
    }
}

#[cfg(not(feature = "std"))]
impl Euclid for f32 {
    #[inline]
    fn div_euclid(&self, v: &f32) -> f32 {
        let q = <f32 as crate::float::FloatCore>::trunc(self / v);
        if self % v < 0.0 {
            return if *v > 0.0 { q - 1.0 } else { q + 1.0 };
        }
        q
    }

    #[inline]
    fn rem_euclid(&self, v: &f32) -> f32 {
        let r = self % v;
        if r < 0.0 {
            r + <f32 as crate::float::FloatCore>::abs(*v)
        } else {
            r
        }
    }
}

#[cfg(not(feature = "std"))]
impl Euclid for f64 {
    #[inline]
    fn div_euclid(&self, v: &f64) -> f64 {
        let q = <f64 as crate::float::FloatCore>::trunc(self / v);
        if self % v < 0.0 {
            return if *v > 0.0 { q - 1.0 } else { q + 1.0 };
        }
        q
    }

    #[inline]
    fn rem_euclid(&self, v: &f64) -> f64 {
        let r = self % v;
        if r < 0.0 {
            r + <f64 as crate::float::FloatCore>::abs(*v)
        } else {
            r
        }
    }
}

#[cfg(has_f128)]
#[cfg(not(feature = "std"))]
impl Euclid for f128 {
    #[inline]
    fn div_euclid(&self, v: &f128) -> f128 {
        let q = <f128 as crate::float::FloatCore>::trunc(self / v);
        if self % v < 0.0 {
            return if *v > 0.0 { q - 1.0 } else { q + 1.0 };
        }
        q
    }

    #[inline]
    fn rem_euclid(&self, v: &f128) -> f128 {
        let r = self % v;
        if r < 0.0 {
            r + <f128 as crate::float::FloatCore>::abs(*v)
        } else {
            r
        }
    }
}

pub trait CheckedEuclid: Euclid {
    /// Performs euclid division, returning `None` on division by zero or if
    /// overflow occurred.
    fn checked_div_euclid(&self, v: &Self) -> Option<Self>;

    /// Finds the euclid remainder of dividing two numbers, returning `None` on
    /// division by zero or if overflow occurred.
    fn checked_rem_euclid(&self, v: &Self) -> Option<Self>;

    /// Returns both the quotient and remainder from checked Euclidean division,
    /// returning `None` on division by zero or if overflow occurred.
    ///
    /// By default, it internally calls both `CheckedEuclid::checked_div_euclid` and `CheckedEuclid::checked_rem_euclid`,
    /// but it can be overridden in order to implement some optimization.
    /// # Examples
    ///
    /// ```
    /// # use num_traits::CheckedEuclid;
    /// let x = 5u8;
    /// let y = 3u8;
    ///
    /// let div = CheckedEuclid::checked_div_euclid(&x, &y);
    /// let rem = CheckedEuclid::checked_rem_euclid(&x, &y);
    ///
    /// assert_eq!(Some((div.unwrap(), rem.unwrap())), CheckedEuclid::checked_div_rem_euclid(&x, &y));
    /// ```
    fn checked_div_rem_euclid(&self, v: &Self) -> Option<(Self, Self)> {
        Some((self.checked_div_euclid(v)?, self.checked_rem_euclid(v)?))
    }
}

macro_rules! checked_euclid_forward_impl {
    ($($t:ty)*) => {$(
        impl CheckedEuclid for $t {
            #[inline]
            fn checked_div_euclid(&self, v: &$t) -> Option<Self> {
                <$t>::checked_div_euclid(*self, *v)
            }

            #[inline]
            fn checked_rem_euclid(&self, v: &$t) -> Option<Self> {
                <$t>::checked_rem_euclid(*self, *v)
            }
        }
    )*}
}

checked_euclid_forward_impl!(isize i8 i16 i32 i64 i128);
checked_euclid_forward_impl!(usize u8 u16 u32 u64 u128);

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
                        let div = Euclid::div_euclid(&x, &y);
                        let rem = Euclid::rem_euclid(&x, &y);
                        assert_eq!(div, 3);
                        assert_eq!(rem, 1);
                        assert_eq!((div, rem), Euclid::div_rem_euclid(&x, &y));
                    }
                )+
            };
        }

        test_euclid!(usize u8 u16 u32 u64);
    }

    #[test]
    fn euclid_signed() {
        macro_rules! test_euclid {
            ($($t:ident)+) => {
                $(
                    {
                        let x: $t = 10;
                        let y: $t = -3;
                        assert_eq!(Euclid::div_euclid(&x, &y), -3);
                        assert_eq!(Euclid::div_euclid(&-x, &y), 4);
                        assert_eq!(Euclid::rem_euclid(&x, &y), 1);
                        assert_eq!(Euclid::rem_euclid(&-x, &y), 2);
                        assert_eq!((Euclid::div_euclid(&x, &y), Euclid::rem_euclid(&x, &y)), Euclid::div_rem_euclid(&x, &y));
                        let x: $t = $t::min_value() + 1;
                        let y: $t = -1;
                        assert_eq!(Euclid::div_euclid(&x, &y), $t::max_value());
                    }
                )+
            };
        }

        test_euclid!(isize i8 i16 i32 i64 i128);
    }

    #[test]
    fn euclid_float() {
        macro_rules! test_euclid {
            ($($t:ident)+) => {
                $(
                    {
                        let x: $t = 12.1;
                        let y: $t = 3.2;
                        assert!(Euclid::div_euclid(&x, &y) * y + Euclid::rem_euclid(&x, &y) - x
                        <= 46.4 * <$t as crate::float::FloatCore>::epsilon());
                        assert!(Euclid::div_euclid(&x, &-y) * -y + Euclid::rem_euclid(&x, &-y) - x
                        <= 46.4 * <$t as crate::float::FloatCore>::epsilon());
                        assert!(Euclid::div_euclid(&-x, &y) * y + Euclid::rem_euclid(&-x, &y) + x
                        <= 46.4 * <$t as crate::float::FloatCore>::epsilon());
                        assert!(Euclid::div_euclid(&-x, &-y) * -y + Euclid::rem_euclid(&-x, &-y) + x
                        <= 46.4 * <$t as crate::float::FloatCore>::epsilon());
                        assert_eq!((Euclid::div_euclid(&x, &y), Euclid::rem_euclid(&x, &y)), Euclid::div_rem_euclid(&x, &y));
                    }
                )+
            };
        }

        #[cfg(has_f16)]
        test_euclid!(f16);

        test_euclid!(f32 f64);

        #[cfg(has_f128)]
        test_euclid!(f128);
    }

    #[test]
    fn euclid_checked() {
        macro_rules! test_euclid_checked {
            ($($t:ident)+) => {
                $(
                    {
                        assert_eq!(CheckedEuclid::checked_div_euclid(&$t::min_value(), &-1), None);
                        assert_eq!(CheckedEuclid::checked_rem_euclid(&$t::min_value(), &-1), None);
                        assert_eq!(CheckedEuclid::checked_div_euclid(&1, &0), None);
                        assert_eq!(CheckedEuclid::checked_rem_euclid(&1, &0), None);
                    }
                )+
            };
        }

        test_euclid_checked!(isize i8 i16 i32 i64 i128);
    }
}
