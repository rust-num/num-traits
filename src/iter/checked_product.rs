use crate::identities::One;
use crate::CheckedMul;

/// Trait to represent types that can be created by multiplying elements of an iterator with overflow checking.
/// This trait should rarely be called directly.
pub trait CheckedProduct<A = Self> : Sized {
    /// Method which takes an iterator and generates Self from the elements by multiplying the items with overflow checking, returning `None` if the multiplication would overflow.
    ///
    /// An empty iterator returns the one value of the type.
    ///
    /// For iterators containing zero, the order of elements may effect whether the result is `None`.
    fn checked_product<I: Iterator<Item = A>>(iter: I) -> Option<Self>;
}

impl<T> CheckedProduct<T> for T
where
    T: CheckedMul + One,
{
    fn checked_product<I: Iterator<Item = Self>>(mut iter: I) -> Option<Self> {
        iter.try_fold(Self::one(), |acc, x| acc.checked_mul(&x))
    }
}

impl<'a, T> CheckedProduct<&'a T> for T
where
    T: CheckedMul + One,
{
    fn checked_product<I: Iterator<Item = &'a Self>>(mut iter: I) -> Option<Self> {
        iter.try_fold(Self::one(), |acc, x| acc.checked_mul(&x))
    }
}

#[test]
fn checked_product_returns_none_instead_of_overflowing() {
    use crate::iter::num_iter::NumIter;

    macro_rules! test_checked_product {
        ($($t:ty)+) => {
            $(
                assert_eq!(None::<$t>, [<$t>::MAX, 2 ].iter().checked_product() );
                assert_eq!(None::<$t>,IntoIterator::into_iter([<$t>::MAX, 2]).checked_product() );
            )+
        };
    }

    test_checked_product!(usize u8 u16 u32 u64 isize i8 i16 i32 i64);
}

#[test]
fn checked_product_returns_one_if_empty() {
    use crate::iter::num_iter::NumIter;

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
    use crate::iter::num_iter::NumIter;

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
    use crate::iter::num_iter::NumIter;

    assert_eq!(None::<u8>, [100u8, 3u8, 0u8].iter().checked_product());
    assert_eq!(Some(0), [0u8, 100u8, 3u8].iter().checked_product());
}
