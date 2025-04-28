use core::num::Wrapping;
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

/// Numbers which have upper and lower bounds
pub trait ConstBounded: ConstLowerBounded + ConstUpperBounded {}

impl<T: ConstLowerBounded + ConstUpperBounded> ConstBounded for T {}

/// Defines an associated constant representing the lower bound of this type
pub trait ConstLowerBounded {
    /// The smallest finite number this type can represent
    const MIN: Self;
}

/// Defines an associated constant representing the upper bound of this type
pub trait ConstUpperBounded {
    /// The largest finite number this type can represent
    const MAX: Self;
}

macro_rules! const_bounded_impl {
    ($t:ty, $min:expr, $max:expr) => {
        impl ConstLowerBounded for $t {
            const MIN: Self = $min;
        }

        impl ConstUpperBounded for $t {
            const MAX: Self = $max;
        }
    };
}

const_bounded_impl!(usize, usize::MIN, usize::MAX);
const_bounded_impl!(u8, u8::MIN, u8::MAX);
const_bounded_impl!(u16, u16::MIN, u16::MAX);
const_bounded_impl!(u32, u32::MIN, u32::MAX);
const_bounded_impl!(u64, u64::MIN, u64::MAX);
const_bounded_impl!(u128, u128::MIN, u128::MAX);

const_bounded_impl!(isize, isize::MIN, isize::MAX);
const_bounded_impl!(i8, i8::MIN, i8::MAX);
const_bounded_impl!(i16, i16::MIN, i16::MAX);
const_bounded_impl!(i32, i32::MIN, i32::MAX);
const_bounded_impl!(i64, i64::MIN, i64::MAX);
const_bounded_impl!(i128, i128::MIN, i128::MAX);

const_bounded_impl!(f32, f32::MIN, f32::MAX);
const_bounded_impl!(f64, f64::MIN, f64::MAX);

impl<T: ConstLowerBounded> ConstLowerBounded for Wrapping<T> {
    const MIN: Self = Wrapping(T::MIN);
}

impl<T: ConstUpperBounded> ConstUpperBounded for Wrapping<T> {
    const MAX: Self = Wrapping(T::MAX);
}

macro_rules! const_lower_bounded_tuple {
    ( $($name:ident)* ) => (
        impl<$($name: ConstLowerBounded,)*> ConstLowerBounded for ($($name,)*) {
            const MIN: Self = ($($name::MIN,)*);
        }
    );
}

macro_rules! const_upper_bounded_tuple {
    ( $($name:ident)* ) => (
        impl<$($name: ConstUpperBounded,)*> ConstUpperBounded for ($($name,)*) {
            const MAX: Self = ($($name::MAX,)*);
        }
    );
}

for_each_tuple!(const_lower_bounded_tuple);
for_each_tuple!(const_upper_bounded_tuple);


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
fn primitives_const_bounded() {
    macro_rules! test_primitives_const_bounded {
        ($($t:ty)+) => {
            $(
                assert_eq!(<$t as ConstLowerBounded>::MIN, <$t>::MIN);
                assert_eq!(<$t as ConstUpperBounded>::MAX, <$t>::MAX);
            )+
        };
    }

    test_primitives_const_bounded!(usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64);
}

#[test]
fn wrapping_const_bounded() {
    macro_rules! test_wrapping_const_bounded {
        ($($t:ty)+) => {
            $(
                assert_eq!(<Wrapping<$t> as ConstLowerBounded>::MIN.0, <$t>::MIN);
                assert_eq!(<Wrapping<$t> as ConstUpperBounded>::MAX.0, <$t>::MAX);
            )+
        };
    }

    test_wrapping_const_bounded!(usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128);
}

#[test]
fn func_call_const_bounded() {
    fn require_const_bounded<T: ConstBounded>(_: &T) -> (T, T) {
        (T::MIN, T::MAX)
    }

    require_const_bounded(&Wrapping(42_u32));
    require_const_bounded(&Wrapping(-42));

    require_const_bounded(&27i32);
    require_const_bounded(&42usize);
    require_const_bounded(&3.1415f32);
}