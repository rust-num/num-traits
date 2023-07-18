use crate::identities::Zero;
use crate::CheckedAdd;

/// Trait to represent types that can be created by summing up an iterator with overflow checking.
/// This trait should rarely be called directly.
pub trait CheckedSum<A = Self>: Sized {
    /// Method which takes an iterator and generates Self from the elements by “summing up” the items with overflow checking, returning `None` if the addition would overflow.
    ///
    /// An empty iterator returns the one value of the type.
    ///
    /// For signed numbers, the order of elements may effect whether the result is `None`.
    fn checked_sum<I: Iterator<Item = A>>(iter: I) -> Option<Self>;
}

impl<T> CheckedSum<T> for T
where
    T: CheckedAdd + Zero,
{
    fn checked_sum<I: Iterator<Item = Self>>(mut iter: I) -> Option<Self> {
        iter.try_fold(Self::zero(), |acc, x| acc.checked_add(&x))
    }
}

impl<'a, T> CheckedSum<&'a T> for T
where
    T: CheckedAdd + Zero,
{
    fn checked_sum<I: Iterator<Item = &'a Self>>(mut iter: I) -> Option<Self> {
        iter.try_fold(Self::zero(), |acc, x| acc.checked_add(&x))
    }
}


#[test]
fn checked_sum_returns_none_instead_of_overflowing() {
    use crate::identities::One;
    use crate::iter::num_iter::NumIter;

    macro_rules! test_checked_sum {
        ($($t:ty)+) => {
            $(
                assert_eq!(None::<$t>, [<$t>::MAX, <$t>::one()].iter().checked_sum() );
                assert_eq!(None::<$t>,IntoIterator::into_iter([<$t>::MAX, <$t>::one()]).checked_sum() );
            )+
        };
    }

    test_checked_sum!(usize u8 u16 u32 u64 isize i8 i16 i32 i64);
}

#[test]
fn checked_sum_returns_zero_if_empty() {
    use crate::iter::num_iter::NumIter;

    macro_rules! test_checked_sum {
        ($($t:ty)+) => {
            $(
                assert_eq!(Some(<$t>::zero()), ([] as [$t; 0]).iter().checked_sum() );
                assert_eq!(Some(<$t>::zero()),IntoIterator::into_iter(([] as [$t; 0])).checked_sum() );
            )+
        };
    }

    test_checked_sum!(usize u8 u16 u32 u64 isize i8 i16 i32 i64);
}

#[test]
fn checked_sum_returns_correct_sum() {
    use crate::iter::num_iter::NumIter;

    macro_rules! test_checked_sum {
        ($($t:ty)+) => {
            $(
                assert_eq!(Some(42), ([40,2] as [$t; 2]).iter().checked_sum() );
                assert_eq!(Some(42),IntoIterator::into_iter(([40,2] as [$t; 2])).checked_sum() );
            )+
        };
    }

    test_checked_sum!(usize u8 u16 u32 u64 isize i8 i16 i32 i64);
}

#[test]
fn checked_sum_adds_left_to_right() {
    use crate::iter::num_iter::NumIter;
    
    assert_eq!(None::<i8>, [120i8, 8i8, -1i8].iter().checked_sum());
    assert_eq!(Some(127), [-1i8, 120i8, 8i8].iter().checked_sum());
}
