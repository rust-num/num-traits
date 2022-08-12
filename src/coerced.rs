pub trait Coerced<F> {
    fn coerce_into(self) -> F;
    fn coerce_from(other: F) -> Self;
}

macro_rules! macro_impl_coerce {
    ($($ty:ty)*) => {
        $(
            impl Coerced<f32> for $ty {
                fn coerce_into(self) -> f32 {
                    self as f32
                }
                fn coerce_from(other: f32) -> Self {
                    other as $ty
                }
            }

            impl Coerced<f64> for $ty {
                fn coerce_into(self) -> f64 {
                    self as f64
                }
                fn coerce_from(other: f64) -> Self {
                    other as $ty
                }
            }
        )*
    }
}

macro_impl_coerce!(f32 f64 i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 usize);
