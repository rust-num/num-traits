use crate::CheckedProduct;
use crate::CheckedSum;

/// An [`Iterator`] blanket implementation that provides extra adaptors and
/// methods.
///
/// This traits defines methods for summing and multiplying together iterators with overflow checking.
pub trait NumIter: Iterator {
    /// Sums the elements of an iterator with overflow checking.
    ///
    /// Takes each element, adds them together, and returns the result or None if the addition would overflow.
    ///
    /// An empty iterator returns the zero value of the type.
    ///
    /// For signed numbers, the order of elements may effect whether the result is `None`.
    fn checked_sum<A, T: CheckedSum<A>>(self) -> Option<T>
    where
        Self: Iterator<Item = A>,
        Self: Sized;

    /// Iterates over the entire iterator, multiplying all the elements with overflow checking.
    ///
    /// Takes each element, multiplies them together, and returns the result or None if the multiplication would overflow.
    ///
    /// An empty iterator returns the one value of the type.
    ///
    /// For iterators containing zero, the order of elements may effect whether the result is `None`.
    fn checked_product<A, T: CheckedProduct<A>>(self) -> Option<T>
    where
        Self: Iterator<Item = A>,
        Self: Sized;
}

impl<I: Iterator> NumIter for I {
    fn checked_sum<A, T: CheckedSum<A>>(self) -> Option<T>
    where
        Self: Iterator<Item = A>,
        Self: Sized,
    {
        CheckedSum::checked_sum(self)
    }

    fn checked_product<A, T: CheckedProduct<A>>(self) -> Option<T>
    where
        Self: Iterator<Item = A>,
        Self: Sized,
    {
        CheckedProduct::checked_product(self)
    }
}
