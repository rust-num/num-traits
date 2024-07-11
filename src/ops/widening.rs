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

#[inline(always)]
const fn split_u128(a: u128) -> (u64, u64) {
    ((a >> 64) as _, (a & 0xFFFF_FFFF_FFFF_FFFF) as _)
}

impl WideningMul<Self> for u128 {
    type Output = Self;

    // l4 = 6edd4bba28970573e250bf2d9c0a78e678099b2cbe4fe173049627b94adc6e00 // x_low  * y_low
    // l5 = 6f8eadccec0b2a496887a6c5e50423418263442505e6c7a8896a4b2c0cedceb0 // x_high * y_low
    // l5 = 6f8eadccec0b2a496887a6c5e5042341f1408fdf2e7dcd1c6bbb0a59a8f84796 // l5 + (l4 >> 64)
    // l6 = 6f7e8d9cabbac9d8e7f7061524334250817263544536271808f9eadbccbdaea0 // x_low  * y_high
    // l6 = 6f7e8d9cabbac9d8e7f706152433425172b2f33373b3f43474b4f53575b5f636 // l6 + (l5 & u64::MAX)
    // l6_high = l6 >> 64
    // l7 = 7030f1b27333f4b57636f7b87939faba9bdb1a5998d8175695d5145392d21151 // x_high * y_high
    // l7 = 7030f1b27333f4b57636f7b87939fabb0b69c82684e3419ffe5cbb1977d63492 // (l5 >> 64) + (l6 >> 64) + l7

    // r0 = (l6 << 64) | (l4 & u64::MAX)
    // r1 = l7 + l6_high;

    #[inline]
    fn widening_mul(self, rhs: Self) -> (Self::Output, Self::Output) {
        const LOW_MASK: u128 = u64::MAX as u128;
        let mut lhs_lo = self & LOW_MASK;
        let mut lhs_hi = self >> 64;
        let mut rhs_lo = rhs & LOW_MASK;
        let mut rhs_hi = rhs >> 64;
        
        let mut l4 = lhs_lo.wrapping_mul(rhs_lo);

        rhs_lo = rhs_lo.wrapping_mul(lhs_hi).wrapping_add(l4.wrapping_shr(64));
        lhs_lo = lhs_lo.wrapping_mul(rhs_hi).wrapping_add(rhs_lo & LOW_MASK);

        lhs_hi = lhs_hi.wrapping_mul(rhs_hi);
        rhs_hi = lhs_lo.wrapping_shr(64);
        rhs_lo >>= 64;

        lhs_hi = lhs_hi.wrapping_add(rhs_lo);

        lhs_lo = lhs_lo.wrapping_shl(64);
        l4 &= LOW_MASK;
        lhs_lo |= l4;

        lhs_hi = lhs_hi.wrapping_add(rhs_hi);

        (lhs_lo, lhs_hi)
    }
}

#[test]
fn test_u128_wrapping() {
    fn widening_mul<T: WideningMul<Output = T>>(a: T, b: T) -> (T, T) {
        a.widening_mul(b)
    }
    assert_eq!(widening_mul(0u128, 0u128), (0, 0));
    assert_eq!(widening_mul(u128::MAX, 1), (u128::MAX, 0));
    assert_eq!(widening_mul(u128::MAX, 2), (u128::MAX - 1, 1));
    assert_eq!(widening_mul(u128::MAX, u128::MAX), (1, u128::MAX - 1));
}
