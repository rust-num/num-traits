pub trait Coerced<F> {
    fn coerce_into(self) -> F;
    fn coerce_from(other: F) -> Self;
}

macro_rules! macro_impl_coerce {
    ($tyy:ty, $($ty:ty)*) => {
        $(
            impl Coerced<$tyy> for $ty {
                fn coerce_into(self) -> $tyy {
                    self as $tyy
                }
                fn coerce_from(other: $tyy) -> Self {
                    other as $ty
                }
            }
        )*
    }
}

macro_rules! macro_impl_coerce2 {
    ($($ty:ty)*) => {
        $(
            macro_impl_coerce!($ty, f32 f64 i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 usize);
        )*
    }
}

macro_impl_coerce2!(f32 f64 i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 usize);
