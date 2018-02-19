/// Unary operator for retrieving the multiplicative inverse, or reciprocal, of a value.
pub trait Inv {
    /// The result after applying the operator.
    type Output;

    /// Returns the multiplicative inverse of `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use num_traits::{Inv, One};
    ///
    /// let x = 7.0;
    /// let y = -0.0;
    /// assert_eq!(x.inv() * x, One::one());
    /// assert_eq!(y.inv() * y, One::one());
    /// ```
    fn inv(self) -> Self::Output;
}

impl Inv for f32 {
    type Output = f32;
    #[inline]
    fn inv(self) -> f32 { 1.0 / self }
}
impl Inv for f64 {
    type Output = f64;
    #[inline]
    fn inv(self) -> f64 { 1.0 / self }
}
impl<'a> Inv for &'a f32 {
    type Output = f32;
    #[inline]
    fn inv(self) -> f32 { 1.0 / *self }
}
impl<'a> Inv for &'a f64 {
    type Output = f64;
    #[inline]
    fn inv(self) -> f64 { 1.0 / *self }
}
