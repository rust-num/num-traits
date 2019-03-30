use core::num::Wrapping;
use core::ops::{Add, Mul};

/// Defines an additive identity element for `Self`.
///
/// # Laws
///
/// ```{.text}
/// a + 0 = a       ∀ a ∈ Self
/// 0 + a = a       ∀ a ∈ Self
/// ```
pub trait Zero: Sized + Add<Self, Output = Self> {
    /// Returns the additive identity element of `Self`, `0`.
    /// # Purity
    ///
    /// This function should return the same result at all times regardless of
    /// external mutable state, for example values stored in TLS or in
    /// `static mut`s.
    // This cannot be an associated constant, because of bignums.
    fn zero() -> Self;

    /// Sets `self` to the additive identity element of `Self`, `0`.
    fn set_zero(&mut self) {
        *self = Zero::zero();
    }

    /// Returns `true` if `self` is equal to the additive identity.
    #[inline]
    fn is_zero(&self) -> bool;
}

/// Supplimentary trait for [`Zero`](trait.Zero.html) types which can be
/// expressed as compile-time constants.
///
/// This is implemented for all primitive types, and should be implemented
/// wherever possible. Implementors must ensure that `ConstZero::ZERO` is
/// the same value produced by [`Zero::zero()`](trait.Zero.html#tymethod.zero).
#[cfg(has_associated_consts)]
pub trait ConstZero: Zero {
    /// Additive identity: see [`Zero::zero()`](trait.Zero.html#tymethod.zero).
    const ZERO: Self;
}

macro_rules! zero_impl {
    ($t:ty, $v:expr) => {
        impl Zero for $t {
            #[inline]
            fn zero() -> $t {
                $v
            }
            #[inline]
            fn is_zero(&self) -> bool {
                *self == $v
            }
        }
    };
}

#[cfg(has_associated_consts)]
macro_rules! zero_const_impl {
    ($t:ty, $v:expr) => {
        zero_impl!($t, $v);
        impl ConstZero for $t {
            const ZERO: $t = $v;
        }
    };
}
#[cfg(not(has_associated_consts))]
macro_rules! zero_const_impl {
    ($t:ty, $v:expr) => { zero_impl!($t, $v); };
}

zero_const_impl!(usize, 0);
zero_const_impl!(u8, 0);
zero_const_impl!(u16, 0);
zero_const_impl!(u32, 0);
zero_const_impl!(u64, 0);
#[cfg(has_i128)]
zero_const_impl!(u128, 0);

zero_const_impl!(isize, 0);
zero_const_impl!(i8, 0);
zero_const_impl!(i16, 0);
zero_const_impl!(i32, 0);
zero_const_impl!(i64, 0);
#[cfg(has_i128)]
zero_const_impl!(i128, 0);

zero_const_impl!(f32, 0.0);
zero_const_impl!(f64, 0.0);

impl<T: Zero> Zero for Wrapping<T>
where
    Wrapping<T>: Add<Output = Wrapping<T>>,
{
    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    fn set_zero(&mut self) {
        self.0.set_zero();
    }

    fn zero() -> Self {
        Wrapping(T::zero())
    }
}

#[cfg(has_associated_consts)]
impl<T: ConstZero> ConstZero for Wrapping<T>
where
    Wrapping<T>: Zero,
{
    const ZERO: Self = Wrapping(T::ZERO);
}

/// Defines a multiplicative identity element for `Self`.
///
/// # Laws
///
/// ```{.text}
/// a * 1 = a       ∀ a ∈ Self
/// 1 * a = a       ∀ a ∈ Self
/// ```
pub trait One: Sized + Mul<Self, Output = Self> {
    /// Returns the multiplicative identity element of `Self`, `1`.
    ///
    /// # Purity
    ///
    /// This function should return the same result at all times regardless of
    /// external mutable state, for example values stored in TLS or in
    /// `static mut`s.
    // This cannot be an associated constant, because of bignums.
    fn one() -> Self;

    /// Sets `self` to the multiplicative identity element of `Self`, `1`.
    fn set_one(&mut self) {
        *self = One::one();
    }

    /// Returns `true` if `self` is equal to the multiplicative identity.
    ///
    /// For performance reasons, it's best to implement this manually.
    /// After a semver bump, this method will be required, and the
    /// `where Self: PartialEq` bound will be removed.
    #[inline]
    fn is_one(&self) -> bool
    where
        Self: PartialEq,
    {
        *self == Self::one()
    }
}

/// Supplimentary trait for [`One`](trait.One.html) types which can be
/// expressed as compile-time constants.
///
/// This is implemented for all primitive types, and should be implemented
/// wherever possible. Implementors must ensure that `ConstOne::ONE` is
/// the same value produced by [`One::one()`](trait.One.html#tymethod.one).
#[cfg(has_associated_consts)]
pub trait ConstOne: One {
    /// Multiplicative identity: see [`One::one`](trait.One.html#tymethod.one).
    const ONE: Self;
}

macro_rules! one_impl {
    ($t:ty, $v:expr) => {
        impl One for $t {
            #[inline]
            fn one() -> $t {
                $v
            }
            #[inline]
            fn is_one(&self) -> bool {
                *self == $v
            }
        }
    };
}

#[cfg(has_associated_consts)]
macro_rules! one_const_impl {
    ($t:ty, $v:expr) => {
        one_impl!($t, $v);
        impl ConstOne for $t {
            const ONE: $t = $v;
        }
    };
}
#[cfg(not(has_associated_consts))]
macro_rules! one_const_impl {
    ($t:ty, $v:expr) => { one_impl!($t, $v); };
}


one_const_impl!(usize, 1);
one_const_impl!(u8, 1);
one_const_impl!(u16, 1);
one_const_impl!(u32, 1);
one_const_impl!(u64, 1);
#[cfg(has_i128)]
one_const_impl!(u128, 1);

one_const_impl!(isize, 1);
one_const_impl!(i8, 1);
one_const_impl!(i16, 1);
one_const_impl!(i32, 1);
one_const_impl!(i64, 1);
#[cfg(has_i128)]
one_const_impl!(i128, 1);

one_const_impl!(f32, 1.0);
one_const_impl!(f64, 1.0);

impl<T: One> One for Wrapping<T>
where
    Wrapping<T>: Mul<Output = Wrapping<T>>,
{
    fn set_one(&mut self) {
        self.0.set_one();
    }

    fn one() -> Self {
        Wrapping(T::one())
    }
}

#[cfg(has_associated_consts)]
impl<T: ConstOne> ConstOne for Wrapping<T>
where
    Wrapping<T>: One,
{
    const ONE: Self = Wrapping(T::ONE);
}

// Some helper functions provided for backwards compatibility.

/// Returns the additive identity, `0`.
#[inline(always)]
pub fn zero<T: Zero>() -> T {
    Zero::zero()
}

/// Returns the multiplicative identity, `1`.
#[inline(always)]
pub fn one<T: One>() -> T {
    One::one()
}

#[test]
#[cfg(has_associated_consts)]
fn const_identies() {
    macro_rules! test_zero_one {
        ($zero:expr, $one:expr; $($t:ty),+) => {
            $(
                assert_eq!(<$t as ConstZero>::ZERO, $zero);
                assert_eq!(<$t as ConstZero>::ZERO, <$t as Zero>::zero());
                assert_eq!(<$t as ConstOne>::ONE, $one);
                assert_eq!(<$t as ConstOne>::ONE, <$t as One>::one());
            )+
        }
    }
    test_zero_one!(0, 1; isize, i8, i16, i32, i64, usize, u8, u16, u32, u64);
    test_zero_one!(0.0, 1.0; f32, f64);
}

#[test]
fn wrapping_identities() {
    macro_rules! test_wrapping_identities {
        ($($t:ty)+) => {
            $(
                assert_eq!(zero::<$t>(), zero::<Wrapping<$t>>().0);
                assert_eq!(one::<$t>(), one::<Wrapping<$t>>().0);
                assert_eq!((0 as $t).is_zero(), Wrapping(0 as $t).is_zero());
                assert_eq!((1 as $t).is_zero(), Wrapping(1 as $t).is_zero());
            )+
        };
    }

    test_wrapping_identities!(isize i8 i16 i32 i64 usize u8 u16 u32 u64);
}

#[test]
fn wrapping_is_zero() {
    fn require_zero<T: Zero>(_: &T) {}
    require_zero(&Wrapping(42));
}
#[test]
fn wrapping_is_one() {
    fn require_one<T: One>(_: &T) {}
    require_one(&Wrapping(42));
}
