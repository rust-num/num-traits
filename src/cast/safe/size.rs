/// Cast between different sized numbers without changing sign.
///
/// The `SizeFrom<T>` trait is similar to `std::convert::From<T>`. However,
/// while `std::convert::From<T>` performs a logical conversion, `SizeFrom<T>`
/// performs a bitwise cast from a number to a number of the same sign but
/// of a possibly different size. `TrimFrom<T>` will **never** change from an
/// integer of one signedness to the other.
///
/// Unless you really know what you are doing, you probably don't want this
/// trait. Instead, you should check out the following traits:
///
///   * `GrowFrom<T>`
///   * `TrimFrom<T>`
///   * `SignCast`
pub trait SizeFrom<T> {
    fn size(value: T) -> Self;
}

/// Cast between different sized numbers without changing sign.
///
/// The `SizeInto<T>` trait is similar to `std::convert::Into<T>`. However,
/// while `std::convert::Into<T>` performs a logical conversion, `SizeInto<T>`
/// performs a bitwise cast from a number to a number of the same sign but
/// of a possibly different size. `TrimInto<T>` will **never** change from an
/// integer of one signedness to the other.
///
/// Unless you really know what you are doing, you probably don't want this
/// trait. Instead, you should check out the following traits:
///
///   * `GrowInto<T>`
///   * `TrimInto<T>`
///   * `SignCast`
pub trait SizeInto<T> {
    fn size(self) -> T;
}

// SizeFrom implies SizeInto
impl<T, U> SizeInto<U> for T where U: SizeFrom<T>
{
    #[inline]
    #[must_use]
    fn size(self) -> U {
        U::size(self)
    }
}

// SizeFrom (and thus SizeInto) is reflexive
impl<T> SizeFrom<T> for T {
    #[inline]
    #[must_use]
    fn size(t: T) -> T { t }
}

macro_rules! size_impl {
    ($from:ty > $into:ty) => (
        impl SizeFrom<$from> for $into {
            #[inline]
            #[must_use]
            fn size(value: $from) -> $into {
                value as $into
            }
        }
    );

    (i128 => $into:ty) => (
        #[cfg(has_i128)]
        size_impl! { i128 > $into }
    );

    (u128 => $into:ty) => (
        #[cfg(has_i128)]
        size_impl! { u128 > $into }
    );

    ($from:ty => i128) => (
        #[cfg(has_i128)]
        size_impl! { $from > i128 }
    );

    ($from:ty => u128) => (
        #[cfg(has_i128)]
        size_impl! { $from > u128 }
    );

    ($from:ty => $into:ty) => (
        size_impl! { $from > $into }
    );

    ($kind:ty, $($next:ty),+) => (
        $(
            size_impl! { $kind => $next }
            size_impl! { $next => $kind }
        )+

        size_impl! { $($next),+ }
    );

    ($kind:ty) => ();
}

size_impl! { usize, u128, u64, u32, u16, u8 }
size_impl! { isize, i128, i64, i32, i16, i8 }
size_impl! { f32, f64 }
