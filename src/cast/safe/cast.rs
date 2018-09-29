/// Cast from one numeric type to another.
///
/// The `CastFrom<T>` trait is similar to `std::convert::From<T>`. However,
/// while `std::convert::From<T>` performs a logical conversion, `CastFrom<T>`
/// performs a bitwise cast from one number to another; possibly of a different
/// size, signedness or even numeric type.
///
/// Unless you really know what you are doing, you probably don't want this
/// trait. Instead, you should check out the following traits:
///
///   * `GrowFrom<T>`
///   * `TrimFrom<T>`
///   * `SignCast`
pub trait CastFrom<T> {
    fn cast(value: T) -> Self;
}

/// Cast from one numeric type to another.
///
/// The `CastInto<T>` trait is similar to `std::convert::Into<T>`. However,
/// while `std::convert::Into<T>` performs a logical conversion, `CastInto<T>`
/// performs a bitwise cast from one number to another; possibly of a different
/// size, signedness or even numeric type.
///
/// Unless you really know what you are doing, you probably don't want this
/// trait. Instead, you should check out the following traits:
///
///   * `GrowInto<T>`
///   * `TrimInto<T>`
///   * `SignCast`
pub trait CastInto<T> {
    fn cast(self) -> T;
}

// CastFrom implies CastInto
impl<T, U> CastInto<U> for T where U: CastFrom<T>
{
    #[inline]
    #[must_use]
    fn cast(self) -> U {
        U::cast(self)
    }
}

// CastFrom (and thus CastInto) is reflexive
impl<T> CastFrom<T> for T {
    #[inline]
    #[must_use]
    fn cast(t: T) -> T { t }
}

macro_rules! cast_impl {
    ($from:ty > $into:ty) => (
        impl CastFrom<$from> for $into {
            #[inline]
            #[must_use]
            fn cast(value: $from) -> $into {
                value as $into
            }
        }
    );

    (i128 => $into:ty) => (
        #[cfg(has_i128)]
        cast_impl! { i128 > $into }
    );

    (u128 => $into:ty) => (
        #[cfg(has_i128)]
        cast_impl! { u128 > $into }
    );

    ($from:ty => i128) => (
        #[cfg(has_i128)]
        cast_impl! { $from > i128 }
    );

    ($from:ty => u128) => (
        #[cfg(has_i128)]
        cast_impl! { $from > u128 }
    );

    ($from:ty => $into:ty) => (
        cast_impl! { $from > $into }
    );

    ($kind:ty, $($next:ty),+) => (
        $(
            cast_impl! { $kind => $next }
            cast_impl! { $next => $kind }
        )+

        cast_impl! { $($next),+ }
    );

    ($kind:ty) => ();
}

cast_impl! {
    usize, u128, u64, u32, u16, u8,
    isize, i128, i64, i32, i16, i8,
    f32, f64
}
