/// Unary operator for retrieving the multiplicative inverse, or reciprocal, of a value.
pub trait Inv {
    /// The result after applying the operator.
    type Output;

    /// Returns the multiplicative inverse of `Self`.
    fn inv(self) -> Self::Output;
}

macro_rules! inv_impl {
    ($t:ty, $out:ty, $fn:expr) => {
        impl<'a> Inv for $t {
            type Output = $out;

            #[inline]
            fn inv(self) -> $out {
                ($fn)(self)
            }
        }
    }
}

#[cfg(feature = "std")]
mod float_impls {
    inv_impl!(f32, f32, f32::recip);
    inv_impl!(f64, f64, f64::recip);
    inv_impl!(&'a f32, f32, f32::recip);
    inv_impl!(&'a f64, f64, f64::recip);
}
