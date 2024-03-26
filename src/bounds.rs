use core::num::*;
use core::{f32, f64};
use core::{i128, i16, i32, i64, i8, isize};
use core::{u128, u16, u32, u64, u8, usize};

/// Numbers which have lower bounds
pub trait LowerBounded {
    /// The smallest finite number this type can represent.
    const MIN: Self;
}

/// Numbers which have upper bounds
pub trait UpperBounded {
    /// The largest finite number this type can represent.
    const MAX: Self;
}

/// Numbers which have upper and lower bounds
pub trait Bounded: UpperBounded + LowerBounded + Sized {
    const MIN: Self = <Self as LowerBounded>::MIN;
    const MAX: Self = <Self as UpperBounded>::MAX;
}

macro_rules! bounded_impl {
    ($t:ty) => {

        impl LowerBounded for $t {
            const MIN: $t = <$t>::MIN;
        }

        impl UpperBounded for $t {
            const MAX: $t = <$t>::MAX;
        }
    };
}

macro_rules! bounded_impl_nonzero {
    ($NonZero:ty, $PossiblyZero:ty) => {

        impl LowerBounded for $NonZero {
            const MIN: $NonZero = unsafe {
                if <$PossiblyZero>::MIN == 0 as $PossiblyZero {
                    Self::new_unchecked(1 as $PossiblyZero)
                } else {
                    Self::new_unchecked(<$PossiblyZero>::MIN)
                }
            };
        }

        impl UpperBounded for $NonZero {
            const MAX: $NonZero = unsafe { Self::new_unchecked(<$PossiblyZero>::MAX) };
        }
    };
}

bounded_impl!(u8);
bounded_impl!(u16);
bounded_impl!(u32);
bounded_impl!(u64);
bounded_impl!(u128);
bounded_impl!(usize);

bounded_impl!(i8);
bounded_impl!(i16);
bounded_impl!(i32);
bounded_impl!(i64);
bounded_impl!(i128);
bounded_impl!(isize);

bounded_impl_nonzero!(NonZeroU8, u8);
bounded_impl_nonzero!(NonZeroU16, u16);
bounded_impl_nonzero!(NonZeroU32, u32);
bounded_impl_nonzero!(NonZeroU64, u64);
bounded_impl_nonzero!(NonZeroU128, u128);
bounded_impl_nonzero!(NonZeroUsize, usize);

bounded_impl_nonzero!(NonZeroI8, i8);
bounded_impl_nonzero!(NonZeroI16, i16);
bounded_impl_nonzero!(NonZeroI32, i32);
bounded_impl_nonzero!(NonZeroI64, i64);
bounded_impl_nonzero!(NonZeroI128, i128);
bounded_impl_nonzero!(NonZeroIsize, isize);

impl<T: LowerBounded> LowerBounded for Wrapping<T> {
    const MIN: Wrapping<T> = Wrapping(T::MIN);
}

impl<T: UpperBounded> UpperBounded for Wrapping<T> {
    const MAX: Wrapping<T> = Wrapping(T::MAX);
}

impl<T: UpperBounded + LowerBounded> Bounded for T {}

bounded_impl!(f32);
bounded_impl!(f64);

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
        impl<$($name: LowerBounded,)*> LowerBounded for ($($name,)*) {
            const MIN: Self = ($($name::MIN,)*);
        }
        
        impl<$($name: UpperBounded,)*> UpperBounded for ($($name,)*) {
            const MAX: Self = ($($name::MAX,)*);
        }
    );
}

for_each_tuple!(bounded_tuple);

#[test]
fn wrapping_bounded() {
    macro_rules! test_wrapping_bounded {
        ($($t:ty)+) => {
            $(
                assert_eq!(<Wrapping<$t> as Bounded>::MIN.0, <$t>::MIN);
                assert_eq!(<Wrapping<$t> as Bounded>::MAX.0, <$t>::MAX);
                assert_eq!(<Wrapping<$t> as Bounded>::MIN.0.wrapping_sub(1), <$t>::MAX);
                assert_eq!(<Wrapping<$t> as Bounded>::MAX.0.wrapping_add(1), <$t>::MIN);
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
                assert_eq!(<Wrapping<$t> as Bounded>::MIN.0, <$t>::MIN);
                assert_eq!(<Wrapping<$t> as Bounded>::MAX.0, <$t>::MAX);
                assert_eq!(<Wrapping<$t> as Bounded>::MIN.0.wrapping_sub(1), <$t>::MAX);
                assert_eq!(<Wrapping<$t> as Bounded>::MAX.0.wrapping_add(1), <$t>::MIN);
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
