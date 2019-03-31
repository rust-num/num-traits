use core::num::Wrapping;
use core::{f32, f64};
#[cfg(has_i128)]
use core::{i128, u128};
use core::{i16, i32, i64, i8, isize};
use core::{u16, u32, u64, u8, usize};

/// Numbers which have upper and lower bounds
pub trait Bounded {
    /// returns the smallest finite number this type can represent
    fn min_value() -> Self;
    /// returns the largest finite number this type can represent
    fn max_value() -> Self;
}

/// Supplimentary trait for [`Bounded`](trait.Bounded.html) types which can be
/// expressed as compile-time constants.
///
/// This is implemented for all primitive types, and should be implemented
/// wherever possible. Implementors must ensure that `ConstBounded::MIN_VALUE`
/// and `ConstBounded::MAX_VALUE` are the same values produced by 
/// [`Bounded::min_value()`](trait.Bounded.html#tymethod.min_value) and
/// [`Bounded::max_value()`](trait.Bounded.html#tymethod.max_value)
/// respectively.
#[cfg(has_associated_consts)]
pub trait ConstBounded: Bounded {
    /// The smallest finite number this type can represent
    const MIN_VALUE: Self;
    /// The largest finite number this type can represent
    const MAX_VALUE: Self;
}

macro_rules! bounded_impl {
    ($t:ty, $min:expr, $max:expr) => {
        impl Bounded for $t {
            #[inline]
            fn min_value() -> $t {
                $min
            }

            #[inline]
            fn max_value() -> $t {
                $max
            }
        }
    };
}

#[cfg(has_associated_consts)]
macro_rules! bounded_const_impl {
    ($t:ty, $min:expr, $max:expr) => {
        bounded_impl!($t, $min, $max);
        impl ConstBounded for $t {
            const MIN_VALUE: $t = $min;
            const MAX_VALUE: $t = $max;
        }
    }
}
#[cfg(not(has_associated_consts))]
macro_rules! bounded_const_impl {
    ($t:ty, $min:expr, $max:expr) => { bounded_impl!($t, $min, $max); };
}

bounded_const_impl!(usize, usize::MIN, usize::MAX);
bounded_const_impl!(u8, u8::MIN, u8::MAX);
bounded_const_impl!(u16, u16::MIN, u16::MAX);
bounded_const_impl!(u32, u32::MIN, u32::MAX);
bounded_const_impl!(u64, u64::MIN, u64::MAX);
#[cfg(has_i128)]
bounded_const_impl!(u128, u128::MIN, u128::MAX);

bounded_const_impl!(isize, isize::MIN, isize::MAX);
bounded_const_impl!(i8, i8::MIN, i8::MAX);
bounded_const_impl!(i16, i16::MIN, i16::MAX);
bounded_const_impl!(i32, i32::MIN, i32::MAX);
bounded_const_impl!(i64, i64::MIN, i64::MAX);
#[cfg(has_i128)]
bounded_const_impl!(i128, i128::MIN, i128::MAX);

bounded_const_impl!(f32, f32::MIN, f32::MAX);
bounded_const_impl!(f64, f64::MIN, f64::MAX);

impl<T: Bounded> Bounded for Wrapping<T> {
    fn min_value() -> Self {
        Wrapping(T::min_value())
    }
    fn max_value() -> Self {
        Wrapping(T::max_value())
    }
}
#[cfg(has_associated_consts)]
impl<T: ConstBounded> ConstBounded for Wrapping<T> {
    const MIN_VALUE: Self = Wrapping(T::MIN_VALUE);
    const MAX_VALUE: Self = Wrapping(T::MAX_VALUE);
}

macro_rules! for_each_tuple_ {
    ( $m:ident !! ) => (
        $m! { }
    );
    ( $m:ident !! $h:ident, $($t:ident,)* ) => (
        $m! { $h $($t)* }
        for_each_tuple_! { $m !! $($t,)* }
    );
}
macro_rules! for_each_tuple {
    ($m:ident) => {
        for_each_tuple_! { $m !! A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, }
    };
}

macro_rules! bounded_tuple {
    ( $($name:ident)* ) => (
        impl<$($name: Bounded,)*> Bounded for ($($name,)*) {
            #[inline]
            fn min_value() -> Self {
                ($($name::min_value(),)*)
            }
            #[inline]
            fn max_value() -> Self {
                ($($name::max_value(),)*)
            }
        }
    );
}
for_each_tuple!(bounded_tuple);

#[cfg(has_associated_consts)]
macro_rules! bounded_const_tuple {
    ( $($name:ident)* ) => (
        impl<$($name: ConstBounded,)*> ConstBounded for ($($name,)*) {
            const MIN_VALUE: Self = ($($name::MIN_VALUE,)*);
            const MAX_VALUE: Self = ($($name::MAX_VALUE,)*);
        }
    );
}
#[cfg(has_associated_consts)]
for_each_tuple!(bounded_const_tuple);

#[test]
fn wrapping_bounded() {
    macro_rules! test_wrapping_bounded {
        ($($t:ty)+) => {
            $(
                assert_eq!(Wrapping::<$t>::min_value().0, <$t>::min_value());
                assert_eq!(Wrapping::<$t>::max_value().0, <$t>::max_value());
            )+
        };
    }

    test_wrapping_bounded!(usize u8 u16 u32 u64 isize i8 i16 i32 i64);
}

#[cfg(has_i128)]
#[test]
fn wrapping_bounded_i128() {
    macro_rules! test_wrapping_bounded {
        ($($t:ty)+) => {
            $(
                assert_eq!(Wrapping::<$t>::min_value().0, <$t>::min_value());
                assert_eq!(Wrapping::<$t>::max_value().0, <$t>::max_value());
            )+
        };
    }

    test_wrapping_bounded!(u128 i128);
}

#[test]
fn wrapping_is_bounded() {
    fn require_bounded<T: Bounded>(_: &T) {}
    require_bounded(&Wrapping(42_u32));
    require_bounded(&Wrapping(-42));
}

#[test]
#[cfg(has_associated_consts)]
fn const_bounded_impl() {
    macro_rules! test_traits_match {
        ($($t:ty),+) => { $(
            assert_eq!(
                <$t as Bounded>::min_value(), 
                <$t as ConstBounded>::MIN_VALUE,
            );
            assert_eq!(
                <$t as Bounded>::max_value(), 
                <$t as ConstBounded>::MAX_VALUE,
            );
        )+};
    }
    test_traits_match!(
        u8, u16, u32, u64, usize, 
        i8, i16, i32, i64, isize,
        f32, f64,
        Wrapping<usize>, Wrapping<isize>,
        (u8, i32, f32, Wrapping<u8>, (i8, ((), u16), Wrapping<i16>, f64))
    );
}
