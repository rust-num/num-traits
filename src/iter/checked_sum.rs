use crate::identities::Zero;
use crate::CheckedAdd;

/// This trait represents types of which an iterator can be summed up with overflow checking.
/// This trait should rarely be called directly.
pub trait CheckedSum<Result = Self> {
    /// Adds the elements of an iterator, returning `None` if an overflow would occur.
    ///
    /// Summing an empty iterator returns a value representing Zero.
    ///
    /// For signed numbers, the order of elements may effect whether the result is `None`.    
    fn checked_sum<I: Iterator<Item = Self>>(iter: I) -> Option<Result>;
}

impl<T> CheckedSum<T> for T
where
    T: CheckedAdd + Zero,
{
    fn checked_sum<I: Iterator<Item = Self>>(mut iter: I) -> Option<Self> {
        iter.try_fold(Self::zero(), |acc, x| acc.checked_add(&x))
    }
}

impl<'a, T> CheckedSum<T> for &'a T
where
    T: CheckedAdd + Sized + Zero,
{
    fn checked_sum<I: Iterator<Item = Self>>(mut iter: I) -> Option<T> {
        iter.try_fold(T::zero(), |acc, x| acc.checked_add(x))
    }
}

///This trait is for iterators that can be summed up with overflow checking.
trait CheckedSumIter<T, Result>: Iterator<Item = T> {
    /// Adds the elements of an iterator, returning `None` if an overflow would occur.
    ///
    /// Summing an empty iterator returns a value representing Zero.
    ///
    /// For signed numbers, the order of elements may effect whether the result is `None`.    
    fn checked_sum(self) -> Option<Result>;
}

impl<Result, T: CheckedSum<Result>, I: Iterator<Item = T>> CheckedSumIter<T, Result> for I {
    fn checked_sum(self) -> Option<Result> {
        T::checked_sum(self)
    }
}

#[test]
fn checked_sum_returns_none_instead_of_overflowing() {
    use crate::identities::One;

    macro_rules! test_checked_sum {
        ($($t:ty)+) => {
            $(
                assert_eq!(None, [<$t>::MAX, <$t>::one()].iter().checked_sum() );
                assert_eq!(None,IntoIterator::into_iter([<$t>::MAX, <$t>::one()]).checked_sum() );
            )+
        };
    }

    test_checked_sum!(usize u8 u16 u32 u64 isize i8 i16 i32 i64);
}

#[test]
fn checked_sum_returns_zero_if_empty() {
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
    assert_eq!(None, [120i8, 8i8, -1i8].iter().checked_sum());
    assert_eq!(Some(127), [-1i8, 120i8, 8i8].iter().checked_sum());
}
