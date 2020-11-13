use core::ops::{Div, Rem};
pub trait DivEuclid: Sized + Div<Self, Output = Self> {
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
    /// use num_traits::DivEuclid;
    ///
    /// let a: i32 = 7;
    /// let b: i32 = 4;
    /// assert_eq!(DivEuclid::div_euclid(a,b), 1); // 7 > 4 * 1
    /// assert_eq!(DivEuclid::div_euclid(-a,b), -2); // -7 >= 4 * -2
    /// assert_eq!(DivEuclid::div_euclid(a,-b), -1); // 7 >= -4 * -1
    /// assert_eq!(DivEuclid::div_euclid(-a,-b), 2); // -7 >= -4 * 2
    /// ```
    fn div_euclid(self, v: Self) -> Self;
}
pub trait RemEuclid: Sized + Rem<Self, Output = Self> {
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
    /// use num_traits::RemEuclid;
    ///
    /// let a: i32 = 7;
    /// let b: i32 = 4;
    /// assert_eq!(RemEuclid::rem_euclid(a,b), 3);
    /// assert_eq!(RemEuclid::rem_euclid(-a,b), 1);
    /// assert_eq!(RemEuclid::rem_euclid(a,-b), 3);
    /// assert_eq!(RemEuclid::rem_euclid(-a,-b), 1);
    /// ```
    fn rem_euclid(self, v: Self) -> Self;
}
macro_rules! div_euclid_int_impl {
    ($trait_name:ident for $($t:ty)*) => {$(
        impl $trait_name for $t {
            #[inline]
            fn div_euclid(self, v: $t) -> Self {
                let q = self / v;
                if self % v < 0 {
                    return if v > 0 { q - 1 } else { q + 1 }
                }
                q
            }
        }
    )*}
}
macro_rules! div_euclid_uint_impl {
    ($trait_name:ident for $($t:ty)*) => {$(
        impl $trait_name for $t {
            #[inline]
            fn div_euclid(self, v: $t) -> Self {
                self / v
            }
        }
    )*}
}
macro_rules! rem_euclid_int_impl {
    ($trait_name:ident for $($t:ty)*) => {$(
        impl $trait_name for $t {
            #[inline]
            fn rem_euclid(self, v: $t) -> Self {
                let r = self % v;
                if r < 0 {
                    if v < 0 {
                        r - v
                    } else {
                        r + v
                    }
                } else {
                    r
                }
            }
        }
    )*}
}
macro_rules! rem_euclid_uint_impl {
    ($trait_name:ident for $($t:ty)*) => {$(
        impl $trait_name for $t {
            #[inline]
            fn rem_euclid(self, v: $t) -> Self {
                self % v
            }
        }
    )*}
}
div_euclid_int_impl!(DivEuclid for i8 i16 i32 i64);
div_euclid_uint_impl!(DivEuclid for isize usize u8 u16 u32 u64);
rem_euclid_int_impl!(RemEuclid for i8 i16 i32 i64);
rem_euclid_uint_impl!(RemEuclid for isize usize u8 u16 u32 u64);
#[cfg(has_i128)]
div_euclid_int_impl!(DivEuclid for i128);
div_euclid_uint_impl!(DivEuclid for u128);
rem_euclid_int_impl!(RemEuclid for i128);
rem_euclid_uint_impl!(RemEuclid for u128);

#[cfg(any(feature = "std", feature = "libm"))]
impl DivEuclid for f32 {
    fn div_euclid(self, v: f32) -> f32 {
        let q = <f32 as ::Float>::trunc(self / v);
        if self % v < 0.0 {
            return if v > 0.0 { q - 1.0 } else { q + 1.0 };
        }
        q
    }
}

#[cfg(any(feature = "std", feature = "libm"))]
impl RemEuclid for f32 {
    fn rem_euclid(self, v: f32) -> f32 {
        let r = self % v;
        if r < 0.0 {
            r + <f32 as ::Float>::abs(v)
        } else {
            r
        }
    }
}

#[cfg(any(feature = "std", feature = "libm"))]
impl DivEuclid for f64 {
    fn div_euclid(self, v: f64) -> f64 {
        let q = <f64 as ::Float>::trunc(self / v);
        if self % v < 0.0 {
            return if v > 0.0 { q - 1.0 } else { q + 1.0 };
        }
        q
    }
}
#[cfg(any(feature = "std", feature = "libm"))]
impl RemEuclid for f64 {
    fn rem_euclid(self, v: f64) -> f64 {
        let r = self % v;
        if r < 0.0 {
            r + <f64 as ::Float>::abs(v)
        } else {
            r
        }
    }
}

pub trait CheckedDivEuclid: DivEuclid {
    /// Performs euclid division that returns `None` instead of panicking on division by zero
    /// and instead of wrapping around on underflow and overflow.
    fn checked_div_euclid(self, v: Self) -> Option<Self>;
}
pub trait CheckedRemEuclid: RemEuclid {
    /// Finds the euclid remainder of dividing two numbers, checking for underflow, overflow and
    /// division by zero. If any of that happens, `None` is returned.
    fn checked_rem_euclid(self, v: Self) -> Option<Self>;
}
macro_rules! checked_div_euclid_int_impl {
    ($trait_name:ident for $($t:ty)*) => {$(
        impl $trait_name for $t {
            #[inline]
            fn checked_div_euclid(self, v: $t) -> Option<$t> {
                if v == 0 || (self == Self::MIN && v == -1) {
                    None
                } else {
                    Some(DivEuclid::div_euclid(self,v))
                }
            }
        }
    )*}
}
macro_rules! checked_div_euclid_uint_impl {
    ($trait_name:ident for $($t:ty)*) => {$(
        impl $trait_name for $t {
            #[inline]
            fn checked_div_euclid(self, v: $t) -> Option<$t> {
                if v == 0{
                    None
                } else {
                    Some(DivEuclid::div_euclid(self,v))
                }
            }
        }
    )*}
}
macro_rules! checked_rem_euclid_int_impl {
    ($trait_name:ident for $($t:ty)*) => {$(
        impl $trait_name for $t {
            #[inline]
            fn checked_rem_euclid(self, v: $t) -> Option<$t> {
                if v == 0 || (self == Self::MIN && v == -1) {
                    None
                } else {
                    Some(RemEuclid::rem_euclid(self,v))
                }
            }
        }
    )*}
}
macro_rules! checked_rem_euclid_uint_impl {
    ($trait_name:ident for $($t:ty)*) => {$(
        impl $trait_name for $t {
            #[inline]
            fn checked_rem_euclid(self, v: $t) -> Option<$t> {
                if v == 0{
                    None
                } else {
                    Some(RemEuclid::rem_euclid(self,v))
                }
            }
        }
    )*}
}
checked_div_euclid_int_impl!(CheckedDivEuclid for i8 i16 i32 i64);
checked_div_euclid_uint_impl!(CheckedDivEuclid for isize usize u8 u16 u32 u64);
checked_rem_euclid_int_impl!(CheckedRemEuclid for i8 i16 i32 i64);
checked_rem_euclid_uint_impl!(CheckedRemEuclid for isize usize u8 u16 u32 u64);
#[cfg(has_i128)]
checked_div_euclid_int_impl!(CheckedDivEuclid for i128);
checked_div_euclid_uint_impl!(CheckedDivEuclid for u128);
checked_rem_euclid_int_impl!(CheckedRemEuclid for i128);
checked_rem_euclid_uint_impl!(CheckedRemEuclid for u128);

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
                        assert_eq!(DivEuclid::div_euclid(x,y),3);
                        assert_eq!(RemEuclid::rem_euclid(x,y),1);
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
                        assert_eq!(DivEuclid::div_euclid(x,y),-3);
                        assert_eq!(DivEuclid::div_euclid(-x,y),4);
                        assert_eq!(RemEuclid::rem_euclid(x,y),1);
                        assert_eq!(RemEuclid::rem_euclid(-x,y),2);
                        let x: $t = $t::MIN+1;
                        let y: $t = -1;
                        assert_eq!(DivEuclid::div_euclid(x,y),$t::MAX);
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
                        assert!(DivEuclid::div_euclid(x,y)*y+RemEuclid::rem_euclid(x,y)-x
                        <=46.4 * $t::EPSILON);
                        assert!(DivEuclid::div_euclid(x,-y)*-y+RemEuclid::rem_euclid(x,-y)-x
                        <= 46.4 * $t::EPSILON);
                        assert!(DivEuclid::div_euclid(-x,y)*y+RemEuclid::rem_euclid(-x,y)-(-x)
                        <= 46.4 * $t::EPSILON);
                        assert!(DivEuclid::div_euclid(-x,-y)*-y+RemEuclid::rem_euclid(-x,-y)-(-x)
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
                        assert_eq!(CheckedDivEuclid::checked_div_euclid($t::MIN,-1),None);
                        assert_eq!(CheckedRemEuclid::checked_rem_euclid($t::MIN,-1),None);
                        assert_eq!(CheckedDivEuclid::checked_div_euclid(1,0),None);
                        assert_eq!(CheckedRemEuclid::checked_rem_euclid(1,0),None);
                    }
                )+
            };
        }

        test_euclid_checked!(i8 i16 i32 i64);
    }
}
