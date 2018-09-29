/// Casts an integer to a different sign without changing its size.
///
/// `SignCast` exposes the cast operator in generic contexts. It is used to
/// perform a signed cast between integers of the same size. The `SignCast`
/// trait will **never** change the size of the integer.
pub trait SignCast {
    type Unsigned;
    type Signed;

    fn unsigned(self) -> Self::Unsigned;
    fn signed(self) -> Self::Signed;
}

macro_rules! sign_impl {
    ( $( $usig:ty : $sign:ty )+ ) => (
        $(
            impl SignCast for $usig {
                type Unsigned = $usig;
                type Signed = $sign;

                #[inline]
                #[must_use]
                fn unsigned(self) -> $usig {
                    self
                }

                #[inline]
                #[must_use]
                fn signed(self) -> $sign {
                    self as $sign
                }
            }

            impl SignCast for $sign {
                type Unsigned = $usig;
                type Signed = $sign;

                #[inline]
                #[must_use]
                fn unsigned(self) -> $usig {
                    self as $usig
                }

                #[inline]
                #[must_use]
                fn signed(self) -> $sign {
                    self
                }
            }
        )+
    )
}

#[cfg(has_i128)]
sign_impl! {
    u128 : i128
}

sign_impl! {
    usize : isize
    u64 : i64
    u32 : i32
    u16 : i16
    u8 : i8
}

#[cfg(test)]
mod tests {
    use super::SignCast;

    #[test]
    fn test_size() {
        // Signed to unsigned cast
        assert_eq!(isize::min_value().unsigned(), isize::min_value() as usize);
        assert_eq!(0isize.unsigned(), 0usize);
        assert_eq!(isize::max_value().unsigned(), isize::max_value() as usize);

        // Signed to signed cast
        assert_eq!(isize::min_value().signed(), isize::min_value());
        assert_eq!(0isize.signed(), 0isize);
        assert_eq!(isize::max_value().signed(), isize::max_value());

        // Unsigned to signed cast
        assert_eq!(usize::min_value().signed(), usize::min_value() as isize);
        assert_eq!(usize::max_value().signed(), usize::max_value() as isize);

        // Unsigned to unsigned cast
        assert_eq!(usize::min_value().unsigned(), usize::min_value());
        assert_eq!(usize::max_value().unsigned(), usize::max_value());

        // Test reciprocity
        assert_eq!(isize::min_value().unsigned().signed(), isize::min_value());
        assert_eq!(isize::max_value().unsigned().signed(), isize::max_value());
        assert_eq!(usize::min_value().signed().unsigned(), usize::min_value());
        assert_eq!(usize::max_value().signed().unsigned(), usize::max_value());
    }

    #[test]
    fn test_128() {
        // Signed to unsigned cast
        assert_eq!(i128::min_value().unsigned(), i128::min_value() as u128);
        assert_eq!(0i128.unsigned(), 0u128);
        assert_eq!(i128::max_value().unsigned(), i128::max_value() as u128);

        // Signed to signed cast
        assert_eq!(i128::min_value().signed(), i128::min_value());
        assert_eq!(0i128.signed(), 0i128);
        assert_eq!(i128::max_value().signed(), i128::max_value());

        // Unsigned to signed cast
        assert_eq!(u128::min_value().signed(), u128::min_value() as i128);
        assert_eq!(u128::max_value().signed(), u128::max_value() as i128);

        // Unsigned to unsigned cast
        assert_eq!(u128::min_value().unsigned(), u128::min_value());
        assert_eq!(u128::max_value().unsigned(), u128::max_value());

        // Test reciprocity
        assert_eq!(i128::min_value().unsigned().signed(), i128::min_value());
        assert_eq!(i128::max_value().unsigned().signed(), i128::max_value());
        assert_eq!(u128::min_value().signed().unsigned(), u128::min_value());
        assert_eq!(u128::max_value().signed().unsigned(), u128::max_value());
    }

    #[test]
    fn test_64() {
        // Signed to unsigned cast
        assert_eq!(i64::min_value().unsigned(), i64::min_value() as u64);
        assert_eq!(0i64.unsigned(), 0u64);
        assert_eq!(i64::max_value().unsigned(), i64::max_value() as u64);

        // Signed to signed cast
        assert_eq!(i64::min_value().signed(), i64::min_value());
        assert_eq!(0i64.signed(), 0i64);
        assert_eq!(i64::max_value().signed(), i64::max_value());

        // Unsigned to signed cast
        assert_eq!(u64::min_value().signed(), u64::min_value() as i64);
        assert_eq!(u64::max_value().signed(), u64::max_value() as i64);

        // Unsigned to unsigned cast
        assert_eq!(u64::min_value().unsigned(), u64::min_value());
        assert_eq!(u64::max_value().unsigned(), u64::max_value());

        // Test reciprocity
        assert_eq!(i64::min_value().unsigned().signed(), i64::min_value());
        assert_eq!(i64::max_value().unsigned().signed(), i64::max_value());
        assert_eq!(u64::min_value().signed().unsigned(), u64::min_value());
        assert_eq!(u64::max_value().signed().unsigned(), u64::max_value());
    }

    #[test]
    fn test_32() {
        // Signed to unsigned cast
        assert_eq!(i32::min_value().unsigned(), i32::min_value() as u32);
        assert_eq!(0i32.unsigned(), 0u32);
        assert_eq!(i32::max_value().unsigned(), i32::max_value() as u32);

        // Signed to signed cast
        assert_eq!(i32::min_value().signed(), i32::min_value());
        assert_eq!(0i32.signed(), 0i32);
        assert_eq!(i32::max_value().signed(), i32::max_value());

        // Unsigned to signed cast
        assert_eq!(u32::min_value().signed(), u32::min_value() as i32);
        assert_eq!(u32::max_value().signed(), u32::max_value() as i32);

        // Unsigned to unsigned cast
        assert_eq!(u32::min_value().unsigned(), u32::min_value());
        assert_eq!(u32::max_value().unsigned(), u32::max_value());

        // Test reciprocity
        assert_eq!(i32::min_value().unsigned().signed(), i32::min_value());
        assert_eq!(i32::max_value().unsigned().signed(), i32::max_value());
        assert_eq!(u32::min_value().signed().unsigned(), u32::min_value());
        assert_eq!(u32::max_value().signed().unsigned(), u32::max_value());
    }

    #[test]
    fn test_16() {
        // Signed to unsigned cast
        assert_eq!(i16::min_value().unsigned(), i16::min_value() as u16);
        assert_eq!(0i16.unsigned(), 0u16);
        assert_eq!(i16::max_value().unsigned(), i16::max_value() as u16);

        // Signed to signed cast
        assert_eq!(i16::min_value().signed(), i16::min_value());
        assert_eq!(0i16.signed(), 0i16);
        assert_eq!(i16::max_value().signed(), i16::max_value());

        // Unsigned to signed cast
        assert_eq!(u16::min_value().signed(), u16::min_value() as i16);
        assert_eq!(u16::max_value().signed(), u16::max_value() as i16);

        // Unsigned to unsigned cast
        assert_eq!(u16::min_value().unsigned(), u16::min_value());
        assert_eq!(u16::max_value().unsigned(), u16::max_value());

        // Test reciprocity
        assert_eq!(i16::min_value().unsigned().signed(), i16::min_value());
        assert_eq!(i16::max_value().unsigned().signed(), i16::max_value());
        assert_eq!(u16::min_value().signed().unsigned(), u16::min_value());
        assert_eq!(u16::max_value().signed().unsigned(), u16::max_value());
    }

    #[test]
    fn test_8() {
        // Signed to unsigned cast
        assert_eq!(i8::min_value().unsigned(), i8::min_value() as u8);
        assert_eq!(0i8.unsigned(), 0u8);
        assert_eq!(i8::max_value().unsigned(), i8::max_value() as u8);

        // Signed to signed cast
        assert_eq!(i8::min_value().signed(), i8::min_value());
        assert_eq!(0i8.signed(), 0i8);
        assert_eq!(i8::max_value().signed(), i8::max_value());

        // Unsigned to signed cast
        assert_eq!(u8::min_value().signed(), u8::min_value() as i8);
        assert_eq!(u8::max_value().signed(), u8::max_value() as i8);

        // Unsigned to unsigned cast
        assert_eq!(u8::min_value().unsigned(), u8::min_value());
        assert_eq!(u8::max_value().unsigned(), u8::max_value());

        // Test reciprocity
        assert_eq!(i8::min_value().unsigned().signed(), i8::min_value());
        assert_eq!(i8::max_value().unsigned().signed(), i8::max_value());
        assert_eq!(u8::min_value().signed().unsigned(), u8::min_value());
        assert_eq!(u8::max_value().signed().unsigned(), u8::max_value());
    }
}
