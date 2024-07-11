macro_rules! widening_impl {
    ($limb:ty, $doublelimb:ty, $test_name:ident) => {
        impl WideningMul<$limb> for $limb {
            type Output = $limb;

            #[inline]
            fn widening_mul(self, rhs: $limb) -> (Self::Output, Self::Output) {
                // SAFETY: overflow will be contained within the wider types
                let wide = (self as $doublelimb).wrapping_mul(rhs as $doublelimb);
                (wide as $limb, (wide >> <$limb>::BITS) as $limb)
            }
        }

        impl WideningMul<&'_ $limb> for $limb {
            type Output = $limb;

            #[inline]
            fn widening_mul(self, rhs: &'_ $limb) -> (Self::Output, Self::Output) {
                WideningMul::<$limb>::widening_mul(self, *rhs)
            }
        }

        impl WideningMul<&'_ $limb> for &'_ $limb {
            type Output = $limb;

            #[inline]
            fn widening_mul(self, rhs: &'_ $limb) -> (Self::Output, Self::Output) {
                WideningMul::<$limb>::widening_mul(*self, *rhs)
            }
        }

        impl WideningMul<$limb> for &'_ $limb {
            type Output = $limb;

            #[inline]
            fn widening_mul(self, rhs: $limb) -> (Self::Output, Self::Output) {
                WideningMul::<$limb>::widening_mul(*self, rhs)
            }
        }

        #[test]
        fn $test_name() {
            fn widening_mul<T: WideningMul<Output = T>>(a: T, b: T) -> (T, T) {
                a.widening_mul(b)
            }
            assert_eq!(widening_mul(0 as $limb, 0 as $limb), (0, 0));
            assert_eq!(widening_mul(<$limb>::MAX, 1), (<$limb>::MAX, 0));
            assert_eq!(widening_mul(<$limb>::MAX, 2), (<$limb>::MAX - 1, 1));
            assert_eq!(
                widening_mul(<$limb>::MAX, <$limb>::MAX),
                (1, <$limb>::MAX - 1)
            );
        }
    };
}

/// Calculates the complete product self * rhs without the possibility to overflow.
pub trait WideningMul<Rhs = Self>: Sized {
    type Output;

    #[must_use]
    fn widening_mul(self, rhs: Rhs) -> (Self::Output, Self::Output);
}

widening_impl!(u8, u16, test_u8_wrapping);
widening_impl!(u16, u32, test_u16_wrapping);
widening_impl!(u32, u64, test_u32_wrapping);
widening_impl!(u64, u128, test_u64_wrapping);

#[cfg(target_pointer_width = "16")]
widening_impl!(usize, u16, test_usize_wrapping);

#[cfg(target_pointer_width = "32")]
widening_impl!(usize, u64, test_usize_wrapping);

#[cfg(target_pointer_width = "64")]
widening_impl!(usize, u128, test_usize_wrapping);
