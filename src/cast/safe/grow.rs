/// Cast from a smaller number to a larger one without changing sign.
///
/// The `GrowFrom<T>` trait is similar to `std::convert::From<T>`. However,
/// while `std::convert::From<T>` performs a logical conversion, `GrowFrom<T>`
/// performs a bitwise cast from a number to a number of the same sign but
/// of a possibly larger size. `GrowFrom<T>` will **never** decrease the size
/// of a number or change from an integer of one signedness to the other.
pub trait GrowFrom<T> {
    #[inline]
    #[must_use]
    fn grow(value: T) -> Self;
}

/// Cast from a smaller number to a larger one without changing sign.
///
/// The `GrowInto<T>` trait is similar to `std::convert::Into<T>`. However,
/// while `std::convert::Into<T>` performs a logical conversion, `GrowInto<T>`
/// performs a bitwise cast from a number to a number of the same sign but
/// of a possibly larger size. `GrowInto<T>` will **never** decrease the size
/// of a number or change from an integer of one signedness to the other.
pub trait GrowInto<T> {
    #[inline]
    #[must_use]
    fn grow(self) -> T;
}

// GrowFrom implies GrowInto
impl<T, U> GrowInto<U> for T where U: GrowFrom<T>
{
    #[inline]
    #[must_use]
    fn grow(self) -> U {
        U::grow(self)
    }
}

// GrowFrom (and thus GrowInto) is reflexive
impl<T> GrowFrom<T> for T {
    #[inline]
    #[must_use]
    fn grow(t: T) -> T { t }
}

macro_rules! grow_impl {
    ($from:ty > $into:ty) => (
        impl GrowFrom<$from> for $into {
            #[inline]
            #[must_use]
            fn grow(value: $from) -> $into {
                value as $into
            }
        }
    );

    (i128 => $into:ty) => (
        #[cfg(has_i128)]
        grow_impl! { i128 > $into }
    );

    (u128 => $into:ty) => (
        #[cfg(has_i128)]
        grow_impl! { u128 > $into }
    );

    ($from:ty => i128) => (
        #[cfg(has_i128)]
        grow_impl! { $from > i128 }
    );

    ($from:ty => u128) => (
        #[cfg(has_i128)]
        grow_impl! { $from > u128 }
    );

    ($from:ty => $into:ty) => (
        grow_impl! { $from > $into }
    );

    ($($from:ty : $($into:ty),+)+) => (
        $( $( grow_impl! { $from => $into } )+ )+
    );
}

#[cfg(target_pointer_width = "64")]
grow_impl! {
    u64: usize
    i64: isize
}

#[cfg(target_pointer_width = "32")]
grow_impl! {
    usize: u32
    isize: i32
}

grow_impl! {
    usize: u128, u64
    isize: i128, i64

    u64: u128
    i64: i128

    u32: usize, u128, u64
    i32: isize, i128, i64

    u16: usize, u128, u64, u32
    i16: isize, i128, i64, i32

    u8: usize, u128, u64, u32, u16
    i8: isize, i128, i64, i32, i16

    f32: f64
}
