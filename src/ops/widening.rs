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

// Implement widening multiplication for all primitive types
widening_impl!(u8, u16, test_u8_widening_mul);
widening_impl!(u16, u32, test_u16_widening_mul);
widening_impl!(u32, u64, test_u32_widening_mul);
widening_impl!(u64, u128, test_u64_widening_mul);

#[cfg(target_pointer_width = "16")]
widening_impl!(usize, u16, test_usize_widening_mul);

#[cfg(target_pointer_width = "32")]
widening_impl!(usize, u64, test_usize_widening_mul);

#[cfg(target_pointer_width = "64")]
widening_impl!(usize, u128, test_usize_widening_mul);

impl WideningMul<Self> for u128 {
    type Output = Self;

    #[allow(clippy::cast_possible_truncation, clippy::similar_names, clippy::cast_lossless)]
    #[inline]
    fn widening_mul(self, rhs: Self) -> (Self::Output, Self::Output) {
        #[inline]
        // Carrying multiplication for u64, computes: lhs * rhs + carry
        const fn carrying_mul(lhs: u64, rhs: u64, carry: u64) -> (u64, u64) {
            // SAFETY: overflow will be contained within the wider types
            let wide = (lhs as u128).wrapping_mul(rhs as u128).wrapping_add(carry as u128);
            (wide as u64, (wide >> u64::BITS) as u64)
        }

        let a = (self >> 64) as u64;
        let b = self as u64;
        let c = (rhs >> 64) as u64;
        let d = rhs as u64;
        let (p1, p2) = WideningMul::widening_mul(b, d);
        let (p2, p31) = carrying_mul(b, c, p2);
        let (p2, p32) = carrying_mul(a, d, p2);
        let (p3, p4_overflow) = p31.overflowing_add(p32);
        let (p3, p4) = carrying_mul(a, c, p3);
        let p4 = p4.wrapping_add(p4_overflow as u64);
        ((p1 as Self) | (p2 as Self) << 64, (p3 as Self) | (p4 as Self) << 64)
    }
}

#[test]
fn test_u128_widening_mul() {
    fn widening_mul<T: WideningMul<Output = T>>(a: T, b: T) -> (T, T) {
        a.widening_mul(b)
    }
    assert_eq!(widening_mul(0u128, 0u128), (0, 0));
    assert_eq!(widening_mul(u128::MAX, 1), (u128::MAX, 0));
    assert_eq!(widening_mul(u128::MAX, 2), (u128::MAX - 1, 1));
    assert_eq!(widening_mul(u128::MAX, u128::MAX), (1, u128::MAX - 1));
}
