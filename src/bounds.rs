use core::num::Wrapping;
use core::num::{
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
    NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
};
use core::{f32, f64};
use core::{i128, i16, i32, i64, i8, isize};
use core::{u128, u16, u32, u64, u8, usize};

/// Numbers which have upper and lower bounds
pub trait Bounded {
    // FIXME (#5527): These should be associated constants
    /// Returns the smallest finite number this type can represent
    fn min_value() -> Self;
    /// Returns the largest finite number this type can represent
    fn max_value() -> Self;
}

/// Numbers which have lower bounds
pub trait LowerBounded {
    /// Returns the smallest finite number this type can represent
    fn min_value() -> Self;
}

// FIXME: With a major version bump, this should be a supertrait instead
impl<T: Bounded> LowerBounded for T {
    fn min_value() -> T {
        Bounded::min_value()
    }
}

/// Numbers which have upper bounds
pub trait UpperBounded {
    /// Returns the largest finite number this type can represent
    fn max_value() -> Self;
}

// FIXME: With a major version bump, this should be a supertrait instead
impl<T: Bounded> UpperBounded for T {
    fn max_value() -> T {
        Bounded::max_value()
    }
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

bounded_impl!(usize, usize::MIN, usize::MAX);
bounded_impl!(u8, u8::MIN, u8::MAX);
bounded_impl!(u16, u16::MIN, u16::MAX);
bounded_impl!(u32, u32::MIN, u32::MAX);
bounded_impl!(u64, u64::MIN, u64::MAX);
bounded_impl!(u128, u128::MIN, u128::MAX);

bounded_impl!(isize, isize::MIN, isize::MAX);
bounded_impl!(i8, i8::MIN, i8::MAX);
bounded_impl!(i16, i16::MIN, i16::MAX);
bounded_impl!(i32, i32::MIN, i32::MAX);
bounded_impl!(i64, i64::MIN, i64::MAX);
bounded_impl!(i128, i128::MIN, i128::MAX);

macro_rules! bounded_impl_nonzero_const {
    ($t:ty, $v:expr, $i:ident) => {
        const $i: $t = match <$t>::new($v) {
            Some(nz) => nz,
            None => panic!("bad nonzero bound!"),
        };
    };
}

macro_rules! bounded_impl_nonzero {
    ($t:ty, $min:expr, $max:expr) => {
        impl Bounded for $t {
            #[inline]
            fn min_value() -> $t {
                // when MSRV is 1.70 we can use $t::MIN
                bounded_impl_nonzero_const!($t, $min, MIN);
                MIN
            }

            #[inline]
            fn max_value() -> $t {
                // when MSRV is 1.70 we can use $t::MAX
                bounded_impl_nonzero_const!($t, $max, MAX);
                MAX
            }
        }
    };
}

bounded_impl_nonzero!(NonZeroUsize, 1, usize::MAX);
bounded_impl_nonzero!(NonZeroU8, 1, u8::MAX);
bounded_impl_nonzero!(NonZeroU16, 1, u16::MAX);
bounded_impl_nonzero!(NonZeroU32, 1, u32::MAX);
bounded_impl_nonzero!(NonZeroU64, 1, u64::MAX);
bounded_impl_nonzero!(NonZeroU128, 1, u128::MAX);

bounded_impl_nonzero!(NonZeroIsize, isize::MIN, isize::MAX);
bounded_impl_nonzero!(NonZeroI8, i8::MIN, i8::MAX);
bounded_impl_nonzero!(NonZeroI16, i16::MIN, i16::MAX);
bounded_impl_nonzero!(NonZeroI32, i32::MIN, i32::MAX);
bounded_impl_nonzero!(NonZeroI64, i64::MIN, i64::MAX);
bounded_impl_nonzero!(NonZeroI128, i128::MIN, i128::MAX);

impl<T: Bounded> Bounded for Wrapping<T> {
    fn min_value() -> Self {
        Wrapping(T::min_value())
    }
    fn max_value() -> Self {
        Wrapping(T::max_value())
    }
}

bounded_impl!(f32, f32::MIN, f32::MAX);

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
bounded_impl!(f64, f64::MIN, f64::MAX);

#[test]
fn wrapping_bounded() {
    macro_rules! test_wrapping_bounded {
        ($($t:ty)+) => {
            $(
                assert_eq!(<Wrapping<$t> as Bounded>::min_value().0, <$t>::min_value());
                assert_eq!(<Wrapping<$t> as Bounded>::max_value().0, <$t>::max_value());
            )+
        };
    }

    test_wrapping_bounded!(usize u8 u16 u32 u64 isize i8 i16 i32 i64);
}

#[test]
fn wrapping_bounded_i128() {
    macro_rules! test_wrapping_bounded {
        ($($t:ty)+) => {
            $(
                assert_eq!(<Wrapping<$t> as Bounded>::min_value().0, <$t>::min_value());
                assert_eq!(<Wrapping<$t> as Bounded>::max_value().0, <$t>::max_value());
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
fn bounded_unsigned_nonzero() {
    macro_rules! test_bounded_impl_unsigned_nonzero {
        ($t:ty, $base_ty:ty) => {
            assert_eq!(<$t as Bounded>::min_value().get(), 1);
            assert_eq!(<$t as Bounded>::max_value().get(), <$base_ty>::MAX);
        };
    }

    test_bounded_impl_unsigned_nonzero!(NonZeroUsize, usize);
    test_bounded_impl_unsigned_nonzero!(NonZeroU8, u8);
    test_bounded_impl_unsigned_nonzero!(NonZeroU16, u16);
    test_bounded_impl_unsigned_nonzero!(NonZeroU32, u32);
    test_bounded_impl_unsigned_nonzero!(NonZeroU64, u64);
    test_bounded_impl_unsigned_nonzero!(NonZeroU128, u128);
}

#[test]
fn bounded_signed_nonzero() {
    macro_rules! test_bounded_impl_signed_nonzero {
        ($t:ty, $base_ty:ty) => {
            assert_eq!(<$t as Bounded>::min_value().get(), <$base_ty>::MIN);
            assert_eq!(<$t as Bounded>::max_value().get(), <$base_ty>::MAX);
        };
    }

    test_bounded_impl_signed_nonzero!(NonZeroIsize, isize);
    test_bounded_impl_signed_nonzero!(NonZeroI8, i8);
    test_bounded_impl_signed_nonzero!(NonZeroI16, i16);
    test_bounded_impl_signed_nonzero!(NonZeroI32, i32);
    test_bounded_impl_signed_nonzero!(NonZeroI64, i64);
    test_bounded_impl_signed_nonzero!(NonZeroI128, i128);
}
