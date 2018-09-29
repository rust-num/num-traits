/// Cast from a larger number to a smaller one without changing sign.
///
/// The `TrimFrom<T>` trait is similar to `std::convert::From<T>`. However,
/// while `std::convert::From<T>` performs a logical conversion, `TrimFrom<T>`
/// performs a bitwise cast from a number to a number of the same sign but
/// of a possibly smaller size. `TrimFrom<T>` will **never** increase the size
/// of a number or change from an integer of one signedness to the other.
pub trait TrimFrom<T> {
    fn trim(value: T) -> Self;
}

/// Cast from a larger number to a smaller one without changing sign.
///
/// The `TrimInto<T>` trait is similar to `std::convert::Into<T>`. However,
/// while `std::convert::Into<T>` performs a logical conversion, `TrimInto<T>`
/// performs a bitwise cast from a number to a number of the same sign but
/// of a possibly smaller size. `TrimInto<T>` will **never** increase the size
/// of a number or change from an integer of one signedness to the other.
pub trait TrimInto<T> {
    fn trim(self) -> T;
}

// TrimFrom implies TrimInto
impl<T, U> TrimInto<U> for T where U: TrimFrom<T>
{
    #[inline]
    #[must_use]
    fn trim(self) -> U {
        U::trim(self)
    }
}

// TrimFrom (and thus TrimInto) is reflexive
impl<T> TrimFrom<T> for T {
    #[inline]
    #[must_use]
    fn trim(t: T) -> T { t }
}

macro_rules! trim_impl {
    ($from:ty > $into:ty) => (
        impl TrimFrom<$from> for $into {
            #[inline]
            #[must_use]
            fn trim(value: $from) -> $into {
                value as $into
            }
        }
    );

    (i128 => $into:ty) => (
        #[cfg(has_i128)]
        trim_impl! { i128 > $into }
    );

    (u128 => $into:ty) => (
        #[cfg(has_i128)]
        trim_impl! { u128 > $into }
    );

    ($from:ty => i128) => (
        #[cfg(has_i128)]
        trim_impl! { $from > i128 }
    );

    ($from:ty => u128) => (
        #[cfg(has_i128)]
        trim_impl! { $from > u128 }
    );

    ($from:ty => $into:ty) => (
        trim_impl! { $from > $into }
    );

    ($($from:ty : $($into:ty),+)+) => (
        $( $( trim_impl! { $from => $into } )+ )+
    );
}

#[cfg(target_pointer_width = "64")]
trim_impl! {
    usize: u64
    isize: i64
}

#[cfg(target_pointer_width = "32")]
trim_impl! {
    u32: usize
    i32: isize
}

trim_impl! {
    usize: u32, u16, u8
    isize: i32, i16, i8

    u128: usize, u64, u32, u16, u8
    i128: isize, i64, i32, i16, i8

    u64: usize, u32, u16, u8
    i64: isize, i32, i16, i8

    u32: u16, u8
    i32: i16, i8

    u16: u8
    i16: i8

    f64: f32
}
