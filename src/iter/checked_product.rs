use crate::identities::One;
use crate::CheckedMul;

/// This trait represents types of which an iterator can be multiplied up with overflow checking.
/// This trait should rarely be called directly.
pub trait CheckedProduct<Result = Self> {
    /// Multiplies up the elements of an iterator, returning `None` if an overflow would occur.
    ///
    /// Multiplies up an empty iterator returns a value representing One.
    ///
    /// If the iterator contains Zero, the order of elements may effect whether the result is `None`.
    fn checked_product<I: Iterator<Item = Self>>(iter: I) -> Option<Result>;
}

impl<T> CheckedProduct<T> for T
where
    T: CheckedMul + One,
{
    fn checked_product<I: Iterator<Item = Self>>(mut iter: I) -> Option<Self> {
        iter.try_fold(Self::one(), |acc, x| acc.checked_mul(&x))
    }
}

impl<'a, T> CheckedProduct<T> for &'a T
where
    T: CheckedMul + Sized + One,
{
    fn checked_product<I: Iterator<Item = Self>>(mut iter: I) -> Option<T> {
        iter.try_fold(T::one(), |acc, x| acc.checked_mul(x))
    }
}

///This trait is for iterators that can be multiplied up with overflow checking.
trait CheckedProductIter<T, Result>: Iterator<Item = T> {
    /// Multiplies up the elements of an iterator, returning `None` if an overflow would occur.
    ///
    /// Multiplies up an empty iterator returns a value representing One.
    ///
    /// If the iterator contains Zero, the order of elements may effect whether the result is `None`.
    fn checked_product(self) -> Option<Result>;
}

impl<Result, T: CheckedProduct<Result>, I: Iterator<Item = T>> CheckedProductIter<T, Result> for I {
    fn checked_product(self) -> Option<Result> {
        T::checked_product(self)
    }
}

#[test]
fn checked_product_returns_none_instead_of_overflowing() {
    macro_rules! test_checked_product {
        ($($t:ty)+) => {
            $(
                assert_eq!(None, [<$t>::MAX, 2 ].iter().checked_product() );
                assert_eq!(None,IntoIterator::into_iter([<$t>::MAX, 2]).checked_product() );
            )+
        };
    }

    test_checked_product!(usize u8 u16 u32 u64 isize i8 i16 i32 i64);
}

#[test]
fn checked_product_returns_one_if_empty() {
    macro_rules! test_checked_product {
        ($($t:ty)+) => {
            $(
                assert_eq!(Some(<$t>::one()), ([] as [$t; 0]).iter().checked_product() );
                assert_eq!(Some(<$t>::one()),IntoIterator::into_iter(([] as [$t; 0])).checked_product() );
            )+
        };
    }

    test_checked_product!(usize u8 u16 u32 u64 isize i8 i16 i32 i64);
}

#[test]
fn checked_product_returns_correct_product() {
    macro_rules! test_checked_product {
        ($($t:ty)+) => {
            $(
                assert_eq!(Some(42), ([3,7,2] as [$t; 3]).iter().checked_product() );
                assert_eq!(Some(42),IntoIterator::into_iter(([3,7,2] as [$t; 3])).checked_product() );
            )+
        };
    }

    test_checked_product!(usize u8 u16 u32 u64 isize i8 i16 i32 i64);
}

#[test]
fn checked_product_multiplies_left_to_right() {
    assert_eq!(None, [100u8, 3u8, 0u8].iter().checked_product());
    assert_eq!(Some(0), [0u8, 100u8, 3u8].iter().checked_product());
}
