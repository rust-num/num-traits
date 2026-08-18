#![forbid(unconditional_recursion)]

use core::{
    cmp::Ordering,
    num::{NonZero, self},
};

/// Returns the ordering between `self` and `other`.
///
/// For some types (particularly floating-point types), there exists a way to
/// induce a canonical ordering on them, but as it might behave in unexpected
/// ways, it isn't the default.
///
/// # Integers
///
/// For integers, this just forwards to the normal [`Ord`] implementation.
///
/// # Floating point numbers
///
/// For floating point numbers this provides an implementation
/// of the `totalOrder` predicate as defined in the IEEE 754 (2008 revision)
/// floating point standard.
pub trait TotalOrder {
    /// Return the ordering between `self` and `other`.
    ///
    /// # Floating point numbers
    ///
    /// Unlike the standard partial comparison between floating point numbers,
    /// this comparison always produces an ordering in accordance to
    /// the `totalOrder` predicate as defined in the IEEE 754 (2008 revision)
    /// floating point standard. The values are ordered in the following sequence:
    ///
    /// - negative quiet NaN
    /// - negative signaling NaN
    /// - negative infinity
    /// - negative numbers
    /// - negative subnormal numbers
    /// - negative zero
    /// - positive zero
    /// - positive subnormal numbers
    /// - positive numbers
    /// - positive infinity
    /// - positive signaling NaN
    /// - positive quiet NaN.
    ///
    /// The ordering established by this function does not always agree with the
    /// [`PartialOrd`] and [`PartialEq`] implementations. For example,
    /// they consider negative and positive zero equal, while `total_cmp`
    /// doesn't.
    ///
    /// The interpretation of the signaling NaN bit follows the definition in
    /// the IEEE 754 standard, which may not match the interpretation by some of
    /// the older, non-conformant (e.g. MIPS) hardware implementations.
    ///
    /// # Examples
    /// ```
    /// use num_traits::ops::total_order::TotalOrder;
    /// use std::cmp::Ordering;
    /// use std::{f32, f64};
    ///
    /// fn check_eq<T: TotalOrder>(x: T, y: T) {
    ///     assert_eq!(x.total_cmp(&y), Ordering::Equal);
    /// }
    ///
    /// check_eq(f64::NAN, f64::NAN);
    /// check_eq(f32::NAN, f32::NAN);
    ///
    /// fn check_lt<T: TotalOrder>(x: T, y: T) {
    ///     assert_eq!(x.total_cmp(&y), Ordering::Less);
    /// }
    ///
    /// check_lt(-f64::NAN, f64::NAN);
    /// check_lt(f64::INFINITY, f64::NAN);
    /// check_lt(-0.0_f64, 0.0_f64);
    /// ```
    fn total_cmp(&self, other: &Self) -> Ordering;
}

impl<T: TotalOrder + ?Sized> TotalOrder for &T {
    #[inline]
    fn total_cmp(&self, other: &Self) -> Ordering {
        (*self).total_cmp(*other)
    }
}

impl<T: TotalOrder + ?Sized> TotalOrder for &mut T {
    #[inline]
    fn total_cmp(&self, other: &Self) -> Ordering {
        (**self).total_cmp(&**other)
    }
}

impl<T: TotalOrder> TotalOrder for Option<T> {
    fn total_cmp(&self, other: &Self) -> Ordering {
        // Implemented the same as `Ord for Option<_>`
        match (self, other) {
            (Some(l), Some(r)) => l.total_cmp(r),
            (Some(_), None) => Ordering::Greater,
            (None, Some(_)) => Ordering::Less,
            (None, None) => Ordering::Equal,
        }
    }
}

#[cfg(has_num_saturating)]
impl<T: TotalOrder> TotalOrder for num::Saturating<T> {
    #[inline]
    fn total_cmp(&self, other: &Self) -> Ordering {
        self.0.total_cmp(&other.0)
    }
}

impl<T: TotalOrder> TotalOrder for num::Wrapping<T> {
    #[inline]
    fn total_cmp(&self, other: &Self) -> Ordering {
        self.0.total_cmp(&other.0)
    }
}

macro_rules! totalorder_float_impl {
    ($T:ident, $I:ident, $U:ident, $bits:expr) => {
        impl TotalOrder for $T {
            #[inline]
            #[cfg(has_total_cmp)]
            fn total_cmp(&self, other: &Self) -> Ordering {
                // Forward to the core implementation
                Self::total_cmp(&self, other)
            }
            #[inline]
            #[cfg(not(has_total_cmp))]
            fn total_cmp(&self, other: &Self) -> Ordering {
                // Backport the core implementation (since 1.62)
                let mut left = self.to_bits() as $I;
                let mut right = other.to_bits() as $I;

                left ^= (((left >> ($bits - 1)) as $U) >> 1) as $I;
                right ^= (((right >> ($bits - 1)) as $U) >> 1) as $I;

                left.cmp(&right)
            }
        }
    };
}
totalorder_float_impl!(f64, i64, u64, 64);
totalorder_float_impl!(f32, i32, u32, 32);

macro_rules! totalorder_impl_via_ord_core {
    ($($t:ty),* $(,)?) => {
        $(
            impl TotalOrder for $t {
                #[inline]
                fn total_cmp(&self, other: &Self) -> Ordering {
                    Self::cmp(self, other)
                }
            }
        )*
    }
}

totalorder_impl_via_ord_core!((), bool, Ordering);

macro_rules! totalorder_impl_zeroable_via_ord_core {
    ($($t:ty),* $(,)?) => {
        totalorder_impl_via_ord_core!($($t,)*);
        $(
            impl TotalOrder for NonZero<$t> {
                #[inline]
                fn total_cmp(&self, other: &Self) -> Ordering {
                    <$t>::cmp(&self.get(), &other.get())
                }
            }
        )*
    }
}

totalorder_impl_zeroable_via_ord_core!(
    char, i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize,
);
