use std::ops::{Add, Sub, Mul, Div, Shl, Shr};

/// Performs addition that returns `None` instead of wrapping around on
/// overflow.
pub trait CheckedAdd: Sized + Add<Self, Output=Self> {
    /// Adds two numbers, checking for overflow. If overflow happens, `None` is
    /// returned.
    fn checked_add(&self, v: &Self) -> Option<Self>;
}

macro_rules! checked_impl {
    ($trait_name:ident, $method:ident, $t:ty) => {
        impl $trait_name for $t {
            #[inline]
            fn $method(&self, v: &$t) -> Option<$t> {
                <$t>::$method(*self, *v)
            }
        }
    }
}

checked_impl!(CheckedAdd, checked_add, u8);
checked_impl!(CheckedAdd, checked_add, u16);
checked_impl!(CheckedAdd, checked_add, u32);
checked_impl!(CheckedAdd, checked_add, u64);
checked_impl!(CheckedAdd, checked_add, usize);

checked_impl!(CheckedAdd, checked_add, i8);
checked_impl!(CheckedAdd, checked_add, i16);
checked_impl!(CheckedAdd, checked_add, i32);
checked_impl!(CheckedAdd, checked_add, i64);
checked_impl!(CheckedAdd, checked_add, isize);

/// Performs subtraction that returns `None` instead of wrapping around on underflow.
pub trait CheckedSub: Sized + Sub<Self, Output=Self> {
    /// Subtracts two numbers, checking for underflow. If underflow happens,
    /// `None` is returned.
    fn checked_sub(&self, v: &Self) -> Option<Self>;
}

checked_impl!(CheckedSub, checked_sub, u8);
checked_impl!(CheckedSub, checked_sub, u16);
checked_impl!(CheckedSub, checked_sub, u32);
checked_impl!(CheckedSub, checked_sub, u64);
checked_impl!(CheckedSub, checked_sub, usize);

checked_impl!(CheckedSub, checked_sub, i8);
checked_impl!(CheckedSub, checked_sub, i16);
checked_impl!(CheckedSub, checked_sub, i32);
checked_impl!(CheckedSub, checked_sub, i64);
checked_impl!(CheckedSub, checked_sub, isize);

/// Performs multiplication that returns `None` instead of wrapping around on underflow or
/// overflow.
pub trait CheckedMul: Sized + Mul<Self, Output=Self> {
    /// Multiplies two numbers, checking for underflow or overflow. If underflow
    /// or overflow happens, `None` is returned.
    fn checked_mul(&self, v: &Self) -> Option<Self>;
}

checked_impl!(CheckedMul, checked_mul, u8);
checked_impl!(CheckedMul, checked_mul, u16);
checked_impl!(CheckedMul, checked_mul, u32);
checked_impl!(CheckedMul, checked_mul, u64);
checked_impl!(CheckedMul, checked_mul, usize);

checked_impl!(CheckedMul, checked_mul, i8);
checked_impl!(CheckedMul, checked_mul, i16);
checked_impl!(CheckedMul, checked_mul, i32);
checked_impl!(CheckedMul, checked_mul, i64);
checked_impl!(CheckedMul, checked_mul, isize);

/// Performs division that returns `None` instead of panicking on division by zero and instead of
/// wrapping around on underflow and overflow.
pub trait CheckedDiv: Sized + Div<Self, Output=Self> {
    /// Divides two numbers, checking for underflow, overflow and division by
    /// zero. If any of that happens, `None` is returned.
    fn checked_div(&self, v: &Self) -> Option<Self>;
}

checked_impl!(CheckedDiv, checked_div, u8);
checked_impl!(CheckedDiv, checked_div, u16);
checked_impl!(CheckedDiv, checked_div, u32);
checked_impl!(CheckedDiv, checked_div, u64);
checked_impl!(CheckedDiv, checked_div, usize);

checked_impl!(CheckedDiv, checked_div, i8);
checked_impl!(CheckedDiv, checked_div, i16);
checked_impl!(CheckedDiv, checked_div, i32);
checked_impl!(CheckedDiv, checked_div, i64);
checked_impl!(CheckedDiv, checked_div, isize);

/// Performs a left shift that returns `None` on overflow.
pub trait CheckedShl<RHS>: Sized + Shl<RHS, Output=Self> {
    /// Shifts a number to the left, checking for overflow. If overflow happens, `None` is
    /// returned.
    fn checked_shl(&self, rhs: &RHS) -> Option<Self>;
}

macro_rules! checked_shift_impl {
    ($trait_name:ident, $method:ident, $rhs:ty, $t:ty) => {
        impl $trait_name<$rhs> for $t {
            #[inline]
            fn $method(&self, rhs: &$rhs) -> Option<$t> {
                // Note the cast to `u32` here: The standard library is somewhat inconsistent here.
                // The `Shl<T>` and `Shr<T>` trait are generic over the right-hand side `T`, but
                // the checked versions of the shifts all operate on `u32`.

                // TODO: Maybe we should use a conversion that can fail here. This would allow us
                // to catch the case where `rhs` exceeds the `u32` accepted by the stdlib, and
                // return a `None` instead.
                <$t>::$method(*self, *rhs as u32)
            }
        }
    }
}

macro_rules! checked_shift_impl_all {
    ($trait_name:ident, $method:ident, $($t:ty)*) => ($(
        checked_shift_impl! { $trait_name, $method, u8   , $t }
        checked_shift_impl! { $trait_name, $method, u16  , $t }
        checked_shift_impl! { $trait_name, $method, u32  , $t }
        checked_shift_impl! { $trait_name, $method, u64  , $t }
        checked_shift_impl! { $trait_name, $method, usize, $t }

        checked_shift_impl! { $trait_name, $method, i8   , $t }
        checked_shift_impl! { $trait_name, $method, i16  , $t }
        checked_shift_impl! { $trait_name, $method, i32  , $t }
        checked_shift_impl! { $trait_name, $method, i64  , $t }
        checked_shift_impl! { $trait_name, $method, isize, $t }
    )*)
}

checked_shift_impl_all!(CheckedShl, checked_shl, u8 u16 u32 u64 usize i8 i16 i32 i64 isize);

/// Performs a right shift that returns `None` on overflow.
pub trait CheckedShr<RHS>: Sized + Shr<RHS, Output=Self> {
    /// Shifts a number to the left, checking for overflow. If overflow happens, `None` is
    /// returned.
    fn checked_shr(&self, rhs: &RHS) -> Option<Self>;
}

checked_shift_impl_all!(CheckedShr, checked_shr, u8 u16 u32 u64 usize i8 i16 i32 i64 isize);
