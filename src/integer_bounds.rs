use core::num::Wrapping;
use core::num::{
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
    NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
};
use core::{f32, f64};
use core::{i128, i16, i32, i64, i8, isize};
use core::{u128, u16, u32, u64, u8, usize};

/// Numbers which have upper and lower bounds
pub trait IntegerBounded : LowerIntegerBounded + UpperIntegerBounded {}

impl<T: LowerIntegerBounded + UpperIntegerBounded> IntegerBounded for T {}

/// Numbers which have lower bounds
pub trait LowerIntegerBounded {
    /// Returns the smallest integer number this type can represent
    const MIN_INTEGER_VALUE: Self;
}

/// Numbers which have upper bounds
pub trait UpperIntegerBounded {
    /// Returns the largest integer number this type can represent
    const MAX_INTEGER_VALUE: Self;
}

// FIXME: With a major version bump, this should be a supertrait instead
const fn max_integer_value<T : IntegerBounded>() -> T {
    T::MAX_INTEGER_VALUE
}

macro_rules! integer_bounded_impl {
    ($t:ty, $min:expr, $max:expr) => {
        impl LowerIntegerBounded for $t {
            const MIN_INTEGER_VALUE: $t = $min;
        }
        impl UpperIntegerBounded for $t {
            const MAX_INTEGER_VALUE: $t = $max;
        }
    };
}

integer_bounded_impl!(usize, usize::MIN, usize::MAX);
integer_bounded_impl!(u8, u8::MIN, u8::MAX);
integer_bounded_impl!(u16, u16::MIN, u16::MAX);
integer_bounded_impl!(u32, u32::MIN, u32::MAX);
integer_bounded_impl!(u64, u64::MIN, u64::MAX);
integer_bounded_impl!(u128, u128::MIN, u128::MAX);

integer_bounded_impl!(isize, isize::MIN, isize::MAX);
integer_bounded_impl!(i8, i8::MIN, i8::MAX);
integer_bounded_impl!(i16, i16::MIN, i16::MAX);
integer_bounded_impl!(i32, i32::MIN, i32::MAX);
integer_bounded_impl!(i64, i64::MIN, i64::MAX);
integer_bounded_impl!(i128, i128::MIN, i128::MAX);


macro_rules! integer_bounded_impl_nonzero {
    ($t:ty, $min:expr, $max:expr) => {
        impl LowerIntegerBounded for $t {
            const MIN_INTEGER_VALUE: $t = match <$t>::new($min) {
                Some(nz) => nz,
                None => panic!("bad nonzero bound!"),
            };
        }
        impl UpperIntegerBounded for $t {
            const MAX_INTEGER_VALUE: $t = match <$t>::new($max) {
                Some(nz) => nz,
                None => panic!("bad nonzero bound!"),
            };
        }
    };
}

integer_bounded_impl_nonzero!(NonZeroUsize, 1, usize::MAX);
integer_bounded_impl_nonzero!(NonZeroU8, 1, u8::MAX);
integer_bounded_impl_nonzero!(NonZeroU16, 1, u16::MAX);
integer_bounded_impl_nonzero!(NonZeroU32, 1, u32::MAX);
integer_bounded_impl_nonzero!(NonZeroU64, 1, u64::MAX);
integer_bounded_impl_nonzero!(NonZeroU128, 1, u128::MAX);

integer_bounded_impl_nonzero!(NonZeroIsize, isize::MIN, isize::MAX);
integer_bounded_impl_nonzero!(NonZeroI8, i8::MIN, i8::MAX);
integer_bounded_impl_nonzero!(NonZeroI16, i16::MIN, i16::MAX);
integer_bounded_impl_nonzero!(NonZeroI32, i32::MIN, i32::MAX);
integer_bounded_impl_nonzero!(NonZeroI64, i64::MIN, i64::MAX);
integer_bounded_impl_nonzero!(NonZeroI128, i128::MIN, i128::MAX);

impl<T: LowerIntegerBounded> LowerIntegerBounded for Wrapping<T> {
    const MIN_INTEGER_VALUE: Self = Wrapping(T::MIN_INTEGER_VALUE);
}
impl<T: UpperIntegerBounded> UpperIntegerBounded for Wrapping<T> {
    const MAX_INTEGER_VALUE: Self = Wrapping(T::MAX_INTEGER_VALUE);
}

integer_bounded_impl!(
    f32,
    -(1 << f32::MANTISSA_DIGITS) as f32,
    (1 << f32::MANTISSA_DIGITS) as f32
);

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

macro_rules! integer_bounded_tuple {
    ( $($name:ident)* ) => (
        impl<$($name: LowerIntegerBounded,)*> LowerIntegerBounded for ($($name,)*) {
            const MIN_INTEGER_VALUE: Self = ($($name::MIN_INTEGER_VALUE,)*);
        }
        impl<$($name: UpperIntegerBounded,)*> UpperIntegerBounded for ($($name,)*) {
            const MAX_INTEGER_VALUE: Self = ($($name::MAX_INTEGER_VALUE,)*);
        }
    );
}

for_each_tuple!(integer_bounded_tuple);
integer_bounded_impl!(
    f64,
    -(1_i64 << f64::MANTISSA_DIGITS) as f64,
    (1_i64 << f64::MANTISSA_DIGITS) as f64
);

#[test]
fn wrapping_integer_bounded() {
    macro_rules! test_wrapping_integer_bounded {
        ($($t:ty)+) => {
            $(
                assert_eq!(<Wrapping<$t> as LowerIntegerBounded>::MIN_INTEGER_VALUE.0, <$t>::MIN_INTEGER_VALUE);
                assert_eq!(<Wrapping<$t> as UpperIntegerBounded>::MAX_INTEGER_VALUE.0, <$t>::MAX_INTEGER_VALUE);
            )+
        };
    }

    test_wrapping_integer_bounded!(usize u8 u16 u32 u64 isize i8 i16 i32 i64);
}

#[test]
fn wrapping_integer_bounded_i128() {
    macro_rules! test_wrapping_integer_bounded {
        ($($t:ty)+) => {
            $(
                assert_eq!(<Wrapping<$t> as LowerIntegerBounded>::MIN_INTEGER_VALUE.0, <$t>::MIN_INTEGER_VALUE);
                assert_eq!(<Wrapping<$t> as UpperIntegerBounded>::MAX_INTEGER_VALUE.0, <$t>::MAX_INTEGER_VALUE);
            )+
        };
    }

    test_wrapping_integer_bounded!(u128 i128);
}

#[test]
fn wrapping_is_integer_bounded() {
    fn require_integer_bounded<T: IntegerBounded>(_: &T) {}
    require_integer_bounded(&Wrapping(42_u32));
    require_integer_bounded(&Wrapping(-42));
}

#[test]
fn integer_bounded_unsigned_nonzero() {
    macro_rules! test_integer_bounded_impl_unsigned_nonzero {
        ($t:ty, $base_ty:ty) => {
            assert_eq!(<$t as LowerIntegerBounded>::MIN_INTEGER_VALUE.get(), 1);
            assert_eq!(
                <$t as UpperIntegerBounded>::MAX_INTEGER_VALUE.get(),
                <$base_ty>::MAX
            );
        };
    }

    test_integer_bounded_impl_unsigned_nonzero!(NonZeroUsize, usize);
    test_integer_bounded_impl_unsigned_nonzero!(NonZeroU8, u8);
    test_integer_bounded_impl_unsigned_nonzero!(NonZeroU16, u16);
    test_integer_bounded_impl_unsigned_nonzero!(NonZeroU32, u32);
    test_integer_bounded_impl_unsigned_nonzero!(NonZeroU64, u64);
    test_integer_bounded_impl_unsigned_nonzero!(NonZeroU128, u128);
}

#[test]
fn integer_bounded_signed_nonzero() {
    macro_rules! test_integer_bounded_impl_signed_nonzero {
        ($t:ty, $base_ty:ty) => {
            assert_eq!(
                <$t as LowerIntegerBounded>::MIN_INTEGER_VALUE.get(),
                <$base_ty>::MIN
            );
            assert_eq!(
                <$t as UpperIntegerBounded>::MAX_INTEGER_VALUE.get(),
                <$base_ty>::MAX
            );
        };
    }

    test_integer_bounded_impl_signed_nonzero!(NonZeroIsize, isize);
    test_integer_bounded_impl_signed_nonzero!(NonZeroI8, i8);
    test_integer_bounded_impl_signed_nonzero!(NonZeroI16, i16);
    test_integer_bounded_impl_signed_nonzero!(NonZeroI32, i32);
    test_integer_bounded_impl_signed_nonzero!(NonZeroI64, i64);
    test_integer_bounded_impl_signed_nonzero!(NonZeroI128, i128);
}

#[test]
fn float_last_integer_values() {
    // f32: integers are exact up to 2^MANTISSA_DIGITS. Adding 1 to the max should
    // not change because spacing is > 1 beyond this value; subtracting 1 should
    // change.
    let f32_min = <f32 as LowerIntegerBounded>::MIN_INTEGER_VALUE;
    let f32_max = <f32 as UpperIntegerBounded>::MAX_INTEGER_VALUE;
    assert_eq!(f32_max + 1.0_f32, f32_max);
    assert_ne!(f32_max - 1.0_f32, f32_max);
    assert_eq!(f32_min - 1.0_f32, f32_min);
    assert_ne!(f32_min + 1.0_f32, f32_min);

    // f64 similarly: integers are exact up to 2^MANTISSA_DIGITS. Adding 1 to the
    // max should not change, subtracting should.
    let f64_min = <f64 as LowerIntegerBounded>::MIN_INTEGER_VALUE;
    let f64_max = <f64 as UpperIntegerBounded>::MAX_INTEGER_VALUE;
    assert_eq!(f64_max + 1.0_f64, f64_max);
    assert_ne!(f64_max - 1.0_f64, f64_max);
    assert_eq!(f64_min - 1.0_f64, f64_min);
    assert_ne!(f64_min + 1.0_f64, f64_min);
}
