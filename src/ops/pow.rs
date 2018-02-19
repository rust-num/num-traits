/// Binary operator for raising a value to a power.
pub trait Pow<RHS> {
    /// The result after applying the operator.
    type Output;

    /// Returns `self` to the power `rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// use num_traits::Pow;
    /// assert_eq!(10.pow(2), 100);
    /// ```
    fn pow(self, rhs: RHS) -> Self::Output;
}

macro_rules! pow_impl {
    ($t:ty, $rhs:ty, $method:ident) => {
        impl Pow<$rhs> for $t {
            type Output = $t;

            #[inline]
            fn pow(self, rhs: $rhs) -> $t {
                <$t>::$method(self, rhs)
            }
        }

        impl<'a> Pow<&'a $rhs> for $t {
            type Output = $t;
            #[inline]
            fn pow(self, rhs: &'a $rhs) -> $t {
                <$t>::$method(self, *rhs)
            }
        }

        impl<'a> Pow<$rhs> for &'a $t {
            type Output = $t;
            #[inline]
            fn pow(self, rhs: $rhs) -> $t {
                <$t>::$method(*self, rhs)
            }
        }

        impl<'a, 'b> Pow<&'a $rhs> for &'b $t {
            type Output = $t;
            #[inline]
            fn pow(self, rhs: &'a $rhs) -> $t {
                <$t>::$method(*self, *rhs)
            }
        }
    }
}

pow_impl!(u8, u32, pow);
pow_impl!(i8, u32, pow);
pow_impl!(u16, u32, pow);
pow_impl!(i16, u32, pow);
pow_impl!(u32, u32, pow);
pow_impl!(i32, u32, pow);
pow_impl!(u64, u32, pow);
pow_impl!(i64, u32, pow);
pow_impl!(usize, u32, pow);
pow_impl!(isize, u32, pow);

#[cfg(feature = "std")]
mod float_impls {
    pow_impl!(f32, i32, powi);
    pow_impl!(f64, i32, powi);
    pow_impl!(f32, f32, powf);
    pow_impl!(f64, f64, powf);
}
