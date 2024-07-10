use core::num::Wrapping;
use core::ops::{Add, Mul};

/// Defines an additive identity element for `Self`.
///
/// # Laws
///
/// ```text
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
    fn is_zero(&self) -> bool;
}

/// Defines an associated constant representing the additive identity element
/// for `Self`.
pub trait ConstZero: Zero {
    /// The additive identity element of `Self`, `0`.
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

        impl ConstZero for $t {
            const ZERO: Self = $v;
        }
    };
}

zero_impl!(usize, 0);
zero_impl!(u8, 0);
zero_impl!(u16, 0);
zero_impl!(u32, 0);
zero_impl!(u64, 0);
zero_impl!(u128, 0);

zero_impl!(isize, 0);
zero_impl!(i8, 0);
zero_impl!(i16, 0);
zero_impl!(i32, 0);
zero_impl!(i64, 0);
zero_impl!(i128, 0);

zero_impl!(f32, 0.0);
zero_impl!(f64, 0.0);

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

impl<T: ConstZero> ConstZero for Wrapping<T>
where
    Wrapping<T>: Add<Output = Wrapping<T>>,
{
    const ZERO: Self = Wrapping(T::ZERO);
}

/// Defines a multiplicative identity element for `Self`.
///
/// # Laws
///
/// ```text
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

/// Defines twice the multiplicative identity element for `Self`.
///
/// # Laws
///
/// ```text
/// a * 2 = a + a      ∀ a ∈ Self
/// 2 * a = a + a      ∀ a ∈ Self
/// ```
pub trait Two: One {
    /// Returns twice the multiplicative identity element of `Self`, `2`.
    ///
    /// # Purity
    ///
    /// This function should return the same result at all times regardless of
    /// external mutable state, for example values stored in TLS or in
    /// `static mut`s.
    // This cannot be an associated constant, because of bignums.
    fn two() -> Self;

    /// Sets `self` to twice the multiplicative identity element of `Self`, `2`.
    fn set_two(&mut self) {
        *self = Two::two();
    }

    /// Returns `true` if `self` is equal to twice the multiplicative identity.
    ///
    /// For performance reasons, it's best to implement this manually.
    /// After a semver bump, this method will be required, and the
    /// `where Self: PartialEq` bound will be removed.
    #[inline]
    fn is_two(&self) -> bool
    where
        Self: PartialEq,
    {
        *self == Self::two()
    }
}

/// Defines an associated constant representing the multiplicative identity
/// element for `Self`.
pub trait ConstOne: One {
    /// The multiplicative identity element of `Self`, `1`.
    const ONE: Self;
}

/// Defines an associated constant representing twice the multiplicative identity
/// element for `Self`.
pub trait ConstTwo: Two {
    /// Twice the multiplicative identity element of `Self`, `2`.
    const TWO: Self;
}

macro_rules! one_two_impl {
    ($t:ty, $one:expr, $two:expr) => {
        impl One for $t {
            #[inline]
            fn one() -> $t {
                $one
            }
            #[inline]
            fn is_one(&self) -> bool {
                *self == $one
            }
        }

        impl ConstOne for $t {
            const ONE: Self = $one;
        }

        impl Two for $t {
            #[inline]
            fn two() -> $t {
                $two
            }
            #[inline]
            fn is_two(&self) -> bool {
                *self == $two
            }
        }

        impl ConstTwo for $t {
            const TWO: Self = $two;
        }
    };
}

one_two_impl!(usize, 1, 2);
one_two_impl!(u8, 1, 2);
one_two_impl!(u16, 1, 2);
one_two_impl!(u32, 1, 2);
one_two_impl!(u64, 1, 2);
one_two_impl!(u128, 1, 2);

one_two_impl!(isize, 1, 2);
one_two_impl!(i8, 1, 2);
one_two_impl!(i16, 1, 2);
one_two_impl!(i32, 1, 2);
one_two_impl!(i64, 1, 2);
one_two_impl!(i128, 1, 2);

one_two_impl!(f32, 1.0, 2.0);
one_two_impl!(f64, 1.0, 2.0);

impl<T: One> One for Wrapping<T>
where
    Wrapping<T>: Mul<Output = Wrapping<T>>,
{
    fn one() -> Self {
        Wrapping(T::one())
    }

    fn set_one(&mut self) {
        self.0.set_one();
    }
}

impl<T: ConstOne> ConstOne for Wrapping<T>
where
    Wrapping<T>: Mul<Output = Wrapping<T>>,
{
    const ONE: Self = Wrapping(T::ONE);
}

impl<T: Two> Two for Wrapping<T>
where
    Wrapping<T>: Mul<Output = Wrapping<T>>,
{
    fn two() -> Self {
        Wrapping(T::two())
    }

    fn set_two(&mut self) {
        self.0.set_two();
    }
}

impl<T: ConstTwo> ConstTwo for Wrapping<T>
where
    Wrapping<T>: Mul<Output = Wrapping<T>>,
{
    const TWO: Self = Wrapping(T::TWO);
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

/// Returns twice the multiplicative identity, `2`.
#[inline(always)]
pub fn two<T: Two>() -> T {
    Two::two()
}

#[test]
fn wrapping_identities() {
    macro_rules! test_wrapping_identities {
        ($($t:ty)+) => {
            $(
                assert_eq!(zero::<$t>(), zero::<Wrapping<$t>>().0);
                assert_eq!(one::<$t>(), one::<Wrapping<$t>>().0);
                assert_eq!(two::<$t>(), two::<Wrapping<$t>>().0);
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

#[test]
fn wrapping_is_two() {
    fn require_two<T: Two>(_: &T) {}
    require_two(&Wrapping(42));
}
