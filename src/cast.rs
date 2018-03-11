use core::{i8, i16, i32, i64, isize};
use core::{u8, u16, u32, u64, usize};
use core::{f32, f64};
use core::mem::size_of;
use core::num::Wrapping;

use identities::Zero;
use bounds::Bounded;
use float::FloatCore;

/// A generic trait for converting a value to a number.
pub trait ToPrimitive {
    /// Converts the value of `self` to an `isize`.
    #[inline]
    fn to_isize(&self) -> Option<isize> {
        self.to_i64().and_then(|x| x.to_isize())
    }

    /// Converts the value of `self` to an `i8`.
    #[inline]
    fn to_i8(&self) -> Option<i8> {
        self.to_i64().and_then(|x| x.to_i8())
    }

    /// Converts the value of `self` to an `i16`.
    #[inline]
    fn to_i16(&self) -> Option<i16> {
        self.to_i64().and_then(|x| x.to_i16())
    }

    /// Converts the value of `self` to an `i32`.
    #[inline]
    fn to_i32(&self) -> Option<i32> {
        self.to_i64().and_then(|x| x.to_i32())
    }

    /// Converts the value of `self` to an `i64`.
    fn to_i64(&self) -> Option<i64>;

    /// Converts the value of `self` to a `usize`.
    #[inline]
    fn to_usize(&self) -> Option<usize> {
        self.to_u64().and_then(|x| x.to_usize())
    }

    /// Converts the value of `self` to an `u8`.
    #[inline]
    fn to_u8(&self) -> Option<u8> {
        self.to_u64().and_then(|x| x.to_u8())
    }

    /// Converts the value of `self` to an `u16`.
    #[inline]
    fn to_u16(&self) -> Option<u16> {
        self.to_u64().and_then(|x| x.to_u16())
    }

    /// Converts the value of `self` to an `u32`.
    #[inline]
    fn to_u32(&self) -> Option<u32> {
        self.to_u64().and_then(|x| x.to_u32())
    }

    /// Converts the value of `self` to an `u64`.
    #[inline]
    fn to_u64(&self) -> Option<u64>;

    /// Converts the value of `self` to an `f32`.
    #[inline]
    fn to_f32(&self) -> Option<f32> {
        self.to_f64().and_then(|x| x.to_f32())
    }

    /// Converts the value of `self` to an `f64`.
    #[inline]
    fn to_f64(&self) -> Option<f64> {
        self.to_i64().and_then(|x| x.to_f64())
    }
}

macro_rules! impl_to_primitive_int_to_int {
    ($SrcT:ty, $DstT:ty, $slf:expr) => (
        {
            if size_of::<$SrcT>() <= size_of::<$DstT>() {
                Some($slf as $DstT)
            } else {
                let n = $slf as i64;
                let min_value: $DstT = Bounded::min_value();
                let max_value: $DstT = Bounded::max_value();
                if min_value as i64 <= n && n <= max_value as i64 {
                    Some($slf as $DstT)
                } else {
                    None
                }
            }
        }
    )
}

macro_rules! impl_to_primitive_int_to_uint {
    ($SrcT:ty, $DstT:ty, $slf:expr) => (
        {
            let zero: $SrcT = Zero::zero();
            let max_value: $DstT = Bounded::max_value();
            if zero <= $slf && $slf as u64 <= max_value as u64 {
                Some($slf as $DstT)
            } else {
                None
            }
        }
    )
}

macro_rules! impl_to_primitive_int {
    ($T:ty) => (
        impl ToPrimitive for $T {
            #[inline]
            fn to_isize(&self) -> Option<isize> { impl_to_primitive_int_to_int!($T, isize, *self) }
            #[inline]
            fn to_i8(&self) -> Option<i8> { impl_to_primitive_int_to_int!($T, i8, *self) }
            #[inline]
            fn to_i16(&self) -> Option<i16> { impl_to_primitive_int_to_int!($T, i16, *self) }
            #[inline]
            fn to_i32(&self) -> Option<i32> { impl_to_primitive_int_to_int!($T, i32, *self) }
            #[inline]
            fn to_i64(&self) -> Option<i64> { impl_to_primitive_int_to_int!($T, i64, *self) }

            #[inline]
            fn to_usize(&self) -> Option<usize> { impl_to_primitive_int_to_uint!($T, usize, *self) }
            #[inline]
            fn to_u8(&self) -> Option<u8> { impl_to_primitive_int_to_uint!($T, u8, *self) }
            #[inline]
            fn to_u16(&self) -> Option<u16> { impl_to_primitive_int_to_uint!($T, u16, *self) }
            #[inline]
            fn to_u32(&self) -> Option<u32> { impl_to_primitive_int_to_uint!($T, u32, *self) }
            #[inline]
            fn to_u64(&self) -> Option<u64> { impl_to_primitive_int_to_uint!($T, u64, *self) }

            #[inline]
            fn to_f32(&self) -> Option<f32> { Some(*self as f32) }
            #[inline]
            fn to_f64(&self) -> Option<f64> { Some(*self as f64) }
        }
    )
}

impl_to_primitive_int!(isize);
impl_to_primitive_int!(i8);
impl_to_primitive_int!(i16);
impl_to_primitive_int!(i32);
impl_to_primitive_int!(i64);

macro_rules! impl_to_primitive_uint_to_int {
    ($DstT:ty, $slf:expr) => (
        {
            let max_value: $DstT = Bounded::max_value();
            if $slf as u64 <= max_value as u64 {
                Some($slf as $DstT)
            } else {
                None
            }
        }
    )
}

macro_rules! impl_to_primitive_uint_to_uint {
    ($SrcT:ty, $DstT:ty, $slf:expr) => (
        {
            if size_of::<$SrcT>() <= size_of::<$DstT>() {
                Some($slf as $DstT)
            } else {
                let zero: $SrcT = Zero::zero();
                let max_value: $DstT = Bounded::max_value();
                if zero <= $slf && $slf as u64 <= max_value as u64 {
                    Some($slf as $DstT)
                } else {
                    None
                }
            }
        }
    )
}

macro_rules! impl_to_primitive_uint {
    ($T:ty) => (
        impl ToPrimitive for $T {
            #[inline]
            fn to_isize(&self) -> Option<isize> { impl_to_primitive_uint_to_int!(isize, *self) }
            #[inline]
            fn to_i8(&self) -> Option<i8> { impl_to_primitive_uint_to_int!(i8, *self) }
            #[inline]
            fn to_i16(&self) -> Option<i16> { impl_to_primitive_uint_to_int!(i16, *self) }
            #[inline]
            fn to_i32(&self) -> Option<i32> { impl_to_primitive_uint_to_int!(i32, *self) }
            #[inline]
            fn to_i64(&self) -> Option<i64> { impl_to_primitive_uint_to_int!(i64, *self) }

            #[inline]
            fn to_usize(&self) -> Option<usize> {
                impl_to_primitive_uint_to_uint!($T, usize, *self)
            }
            #[inline]
            fn to_u8(&self) -> Option<u8> { impl_to_primitive_uint_to_uint!($T, u8, *self) }
            #[inline]
            fn to_u16(&self) -> Option<u16> { impl_to_primitive_uint_to_uint!($T, u16, *self) }
            #[inline]
            fn to_u32(&self) -> Option<u32> { impl_to_primitive_uint_to_uint!($T, u32, *self) }
            #[inline]
            fn to_u64(&self) -> Option<u64> { impl_to_primitive_uint_to_uint!($T, u64, *self) }

            #[inline]
            fn to_f32(&self) -> Option<f32> { Some(*self as f32) }
            #[inline]
            fn to_f64(&self) -> Option<f64> { Some(*self as f64) }
        }
    )
}

impl_to_primitive_uint!(usize);
impl_to_primitive_uint!(u8);
impl_to_primitive_uint!(u16);
impl_to_primitive_uint!(u32);
impl_to_primitive_uint!(u64);

macro_rules! impl_to_primitive_float_to_float {
    ($SrcT:ident, $DstT:ident, $slf:expr) => ({
        // Only finite values that are reducing size need to worry about overflow.
        if size_of::<$SrcT>() > size_of::<$DstT>() && FloatCore::is_finite($slf) {
            let n = $slf as f64;
            if n < $DstT::MIN as f64 || n > $DstT::MAX as f64 {
                return None;
            }
        }
        // We can safely cast NaN, +-inf, and finite values in range.
        Some($slf as $DstT)
    })
}

macro_rules! impl_to_primitive_float_to_signed_int {
    ($SrcT:ident, $DstT:ident, $slf:expr) => ({
        let t = $slf.trunc(); // round toward zero.
        // MIN is a power of two, which we can cast and compare directly.
        if t >= $DstT::MIN as $SrcT {
            // The mantissa might not be able to represent all digits of MAX.
            let sig_bits = size_of::<$DstT>() as u32 * 8 - 1;
            let max = if sig_bits > $SrcT::MANTISSA_DIGITS {
                let lost_bits = sig_bits - $SrcT::MANTISSA_DIGITS;
                $DstT::MAX & !((1 << lost_bits) - 1)
            } else {
                $DstT::MAX
            };
            if t <= max as $SrcT {
                return Some($slf as $DstT);
            }
        }
        None
    })
}

macro_rules! impl_to_primitive_float_to_unsigned_int {
    ($SrcT:ident, $DstT:ident, $slf:expr) => ({
        let t = $slf.trunc(); // round toward zero.
        if t >= 0.0 {
            // The mantissa might not be able to represent all digits of MAX.
            let sig_bits = size_of::<$DstT>() as u32 * 8;
            let max = if sig_bits > $SrcT::MANTISSA_DIGITS {
                let lost_bits = sig_bits - $SrcT::MANTISSA_DIGITS;
                $DstT::MAX & !((1 << lost_bits) - 1)
            } else {
                $DstT::MAX
            };
            if t <= max as $SrcT {
                return Some($slf as $DstT);
            }
        }
        None
    })
}

macro_rules! impl_to_primitive_float {
    ($T:ident) => (
        impl ToPrimitive for $T {
            #[inline]
            fn to_isize(&self) -> Option<isize> {
                impl_to_primitive_float_to_signed_int!($T, isize, *self)
            }
            #[inline]
            fn to_i8(&self) -> Option<i8> {
                impl_to_primitive_float_to_signed_int!($T, i8, *self)
            }
            #[inline]
            fn to_i16(&self) -> Option<i16> {
                impl_to_primitive_float_to_signed_int!($T, i16, *self)
            }
            #[inline]
            fn to_i32(&self) -> Option<i32> {
                impl_to_primitive_float_to_signed_int!($T, i32, *self)
            }
            #[inline]
            fn to_i64(&self) -> Option<i64> {
                impl_to_primitive_float_to_signed_int!($T, i64, *self)
            }

            #[inline]
            fn to_usize(&self) -> Option<usize> {
                impl_to_primitive_float_to_unsigned_int!($T, usize, *self)
            }
            #[inline]
            fn to_u8(&self) -> Option<u8> {
                impl_to_primitive_float_to_unsigned_int!($T, u8, *self)
            }
            #[inline]
            fn to_u16(&self) -> Option<u16> {
                impl_to_primitive_float_to_unsigned_int!($T, u16, *self)
            }
            #[inline]
            fn to_u32(&self) -> Option<u32> {
                impl_to_primitive_float_to_unsigned_int!($T, u32, *self)
            }
            #[inline]
            fn to_u64(&self) -> Option<u64> {
                impl_to_primitive_float_to_unsigned_int!($T, u64, *self)
            }

            #[inline]
            fn to_f32(&self) -> Option<f32> {
                impl_to_primitive_float_to_float!($T, f32, *self)
            }
            #[inline]
            fn to_f64(&self) -> Option<f64> {
                impl_to_primitive_float_to_float!($T, f64, *self)
            }
        }
    )
}

impl_to_primitive_float!(f32);
impl_to_primitive_float!(f64);

/// A generic trait for converting a number to a value.
pub trait FromPrimitive: Sized {
    /// Convert an `isize` to return an optional value of this type. If the
    /// value cannot be represented by this value, the `None` is returned.
    #[inline]
    fn from_isize(n: isize) -> Option<Self> {
        FromPrimitive::from_i64(n as i64)
    }

    /// Convert an `i8` to return an optional value of this type. If the
    /// type cannot be represented by this value, the `None` is returned.
    #[inline]
    fn from_i8(n: i8) -> Option<Self> {
        FromPrimitive::from_i64(n as i64)
    }

    /// Convert an `i16` to return an optional value of this type. If the
    /// type cannot be represented by this value, the `None` is returned.
    #[inline]
    fn from_i16(n: i16) -> Option<Self> {
        FromPrimitive::from_i64(n as i64)
    }

    /// Convert an `i32` to return an optional value of this type. If the
    /// type cannot be represented by this value, the `None` is returned.
    #[inline]
    fn from_i32(n: i32) -> Option<Self> {
        FromPrimitive::from_i64(n as i64)
    }

    /// Convert an `i64` to return an optional value of this type. If the
    /// type cannot be represented by this value, the `None` is returned.
    fn from_i64(n: i64) -> Option<Self>;

    /// Convert a `usize` to return an optional value of this type. If the
    /// type cannot be represented by this value, the `None` is returned.
    #[inline]
    fn from_usize(n: usize) -> Option<Self> {
        FromPrimitive::from_u64(n as u64)
    }

    /// Convert an `u8` to return an optional value of this type. If the
    /// type cannot be represented by this value, the `None` is returned.
    #[inline]
    fn from_u8(n: u8) -> Option<Self> {
        FromPrimitive::from_u64(n as u64)
    }

    /// Convert an `u16` to return an optional value of this type. If the
    /// type cannot be represented by this value, the `None` is returned.
    #[inline]
    fn from_u16(n: u16) -> Option<Self> {
        FromPrimitive::from_u64(n as u64)
    }

    /// Convert an `u32` to return an optional value of this type. If the
    /// type cannot be represented by this value, the `None` is returned.
    #[inline]
    fn from_u32(n: u32) -> Option<Self> {
        FromPrimitive::from_u64(n as u64)
    }

    /// Convert an `u64` to return an optional value of this type. If the
    /// type cannot be represented by this value, the `None` is returned.
    fn from_u64(n: u64) -> Option<Self>;

    /// Convert a `f32` to return an optional value of this type. If the
    /// type cannot be represented by this value, the `None` is returned.
    #[inline]
    fn from_f32(n: f32) -> Option<Self> {
        FromPrimitive::from_f64(n as f64)
    }

    /// Convert a `f64` to return an optional value of this type. If the
    /// type cannot be represented by this value, the `None` is returned.
    #[inline]
    fn from_f64(n: f64) -> Option<Self> {
        FromPrimitive::from_i64(n as i64)
    }
}

macro_rules! impl_from_primitive {
    ($T:ty, $to_ty:ident) => (
        #[allow(deprecated)]
        impl FromPrimitive for $T {
            #[inline] fn from_i8(n: i8) -> Option<$T> { n.$to_ty() }
            #[inline] fn from_i16(n: i16) -> Option<$T> { n.$to_ty() }
            #[inline] fn from_i32(n: i32) -> Option<$T> { n.$to_ty() }
            #[inline] fn from_i64(n: i64) -> Option<$T> { n.$to_ty() }

            #[inline] fn from_u8(n: u8) -> Option<$T> { n.$to_ty() }
            #[inline] fn from_u16(n: u16) -> Option<$T> { n.$to_ty() }
            #[inline] fn from_u32(n: u32) -> Option<$T> { n.$to_ty() }
            #[inline] fn from_u64(n: u64) -> Option<$T> { n.$to_ty() }

            #[inline] fn from_f32(n: f32) -> Option<$T> { n.$to_ty() }
            #[inline] fn from_f64(n: f64) -> Option<$T> { n.$to_ty() }
        }
    )
}

impl_from_primitive!(isize, to_isize);
impl_from_primitive!(i8,    to_i8);
impl_from_primitive!(i16,   to_i16);
impl_from_primitive!(i32,   to_i32);
impl_from_primitive!(i64,   to_i64);
impl_from_primitive!(usize, to_usize);
impl_from_primitive!(u8,    to_u8);
impl_from_primitive!(u16,   to_u16);
impl_from_primitive!(u32,   to_u32);
impl_from_primitive!(u64,   to_u64);
impl_from_primitive!(f32,   to_f32);
impl_from_primitive!(f64,   to_f64);


impl<T: ToPrimitive> ToPrimitive for Wrapping<T> {
    fn to_i64(&self) -> Option<i64> { self.0.to_i64() }
    fn to_u64(&self) -> Option<u64> { self.0.to_u64() }
}
impl<T: FromPrimitive> FromPrimitive for Wrapping<T> {
    fn from_u64(n: u64) -> Option<Self> { T::from_u64(n).map(Wrapping) }
    fn from_i64(n: i64) -> Option<Self> { T::from_i64(n).map(Wrapping) }
}


/// Cast from one machine scalar to another.
///
/// # Examples
///
/// ```
/// # use num_traits as num;
/// let twenty: f32 = num::cast(0x14).unwrap();
/// assert_eq!(twenty, 20f32);
/// ```
///
#[inline]
pub fn cast<T: NumCast, U: NumCast>(n: T) -> Option<U> {
    NumCast::from(n)
}

/// An interface for casting between machine scalars.
pub trait NumCast: Sized + ToPrimitive {
    /// Creates a number from another value that can be converted into
    /// a primitive via the `ToPrimitive` trait.
    fn from<T: ToPrimitive>(n: T) -> Option<Self>;
}

macro_rules! impl_num_cast {
    ($T:ty, $conv:ident) => (
        impl NumCast for $T {
            #[inline]
            #[allow(deprecated)]
            fn from<N: ToPrimitive>(n: N) -> Option<$T> {
                // `$conv` could be generated using `concat_idents!`, but that
                // macro seems to be broken at the moment
                n.$conv()
            }
        }
    )
}

impl_num_cast!(u8,    to_u8);
impl_num_cast!(u16,   to_u16);
impl_num_cast!(u32,   to_u32);
impl_num_cast!(u64,   to_u64);
impl_num_cast!(usize, to_usize);
impl_num_cast!(i8,    to_i8);
impl_num_cast!(i16,   to_i16);
impl_num_cast!(i32,   to_i32);
impl_num_cast!(i64,   to_i64);
impl_num_cast!(isize, to_isize);
impl_num_cast!(f32,   to_f32);
impl_num_cast!(f64,   to_f64);

impl<T: NumCast> NumCast for Wrapping<T> {
    fn from<U: ToPrimitive>(n: U) -> Option<Self> {
        T::from(n).map(Wrapping)
    }
}

/// A generic interface for casting between machine scalars with the
/// `as` operator, which admits narrowing and precision loss.
/// Implementers of this trait AsPrimitive should behave like a primitive
/// numeric type (e.g. a newtype around another primitive), and the
/// intended conversion must never fail.
///
/// # Examples
///
/// ```
/// # use num_traits::AsPrimitive;
/// let three: i32 = (3.14159265f32).as_();
/// assert_eq!(three, 3);
/// ```
/// 
/// # Safety
/// 
/// Currently, some uses of the `as` operator are not entirely safe.
/// In particular, it is undefined behavior if:
/// 
/// - A truncated floating point value cannot fit in the target integer
///   type ([#10184](https://github.com/rust-lang/rust/issues/10184));
/// 
/// ```ignore
/// # use num_traits::AsPrimitive;
/// let x: u8 = (1.04E+17).as_(); // UB
/// ```
/// 
/// - Or a floating point value does not fit in another floating
///   point type ([#15536](https://github.com/rust-lang/rust/issues/15536)).
///
/// ```ignore
/// # use num_traits::AsPrimitive;
/// let x: f32 = (1e300f64).as_(); // UB
/// ```
/// 
pub trait AsPrimitive<T>: 'static + Copy
where
    T: 'static + Copy
{
    /// Convert a value to another, using the `as` operator.
    fn as_(self) -> T;
}

macro_rules! impl_as_primitive {
    ($T: ty => $( $U: ty ),* ) => {
        $(
        impl AsPrimitive<$U> for $T {
            #[inline] fn as_(self) -> $U { self as $U }
        }
        )*
    };
}

impl_as_primitive!(u8 => char, u8, i8, u16, i16, u32, i32, u64, isize, usize, i64, f32, f64);
impl_as_primitive!(i8 => u8, i8, u16, i16, u32, i32, u64, isize, usize, i64, f32, f64);
impl_as_primitive!(u16 => u8, i8, u16, i16, u32, i32, u64, isize, usize, i64, f32, f64);
impl_as_primitive!(i16 => u8, i8, u16, i16, u32, i32, u64, isize, usize, i64, f32, f64);
impl_as_primitive!(u32 => u8, i8, u16, i16, u32, i32, u64, isize, usize, i64, f32, f64);
impl_as_primitive!(i32 => u8, i8, u16, i16, u32, i32, u64, isize, usize, i64, f32, f64);
impl_as_primitive!(u64 => u8, i8, u16, i16, u32, i32, u64, isize, usize, i64, f32, f64);
impl_as_primitive!(i64 => u8, i8, u16, i16, u32, i32, u64, isize, usize, i64, f32, f64);
impl_as_primitive!(usize => u8, i8, u16, i16, u32, i32, u64, isize, usize, i64, f32, f64);
impl_as_primitive!(isize => u8, i8, u16, i16, u32, i32, u64, isize, usize, i64, f32, f64);
impl_as_primitive!(f32 => u8, i8, u16, i16, u32, i32, u64, isize, usize, i64, f32, f64);
impl_as_primitive!(f64 => u8, i8, u16, i16, u32, i32, u64, isize, usize, i64, f32, f64);
impl_as_primitive!(char => char, u8, i8, u16, i16, u32, i32, u64, isize, usize, i64);
impl_as_primitive!(bool => u8, i8, u16, i16, u32, i32, u64, isize, usize, i64);

#[test]
fn to_primitive_float() {
    use core::f32;
    use core::f64;

    let f32_toolarge = 1e39f64;
    assert_eq!(f32_toolarge.to_f32(), None);
    assert_eq!((f32::MAX as f64).to_f32(), Some(f32::MAX));
    assert_eq!((-f32::MAX as f64).to_f32(), Some(-f32::MAX));
    assert_eq!(f64::INFINITY.to_f32(), Some(f32::INFINITY));
    assert_eq!((f64::NEG_INFINITY).to_f32(), Some(f32::NEG_INFINITY));
    assert!((f64::NAN).to_f32().map_or(false, |f| f.is_nan()));
}

#[test]
fn wrapping_to_primitive() {
    macro_rules! test_wrapping_to_primitive {
        ($($t:ty)+) => {
            $({
                let i: $t = 0;
                let w = Wrapping(i);
                assert_eq!(i.to_u8(),    w.to_u8());
                assert_eq!(i.to_u16(),   w.to_u16());
                assert_eq!(i.to_u32(),   w.to_u32());
                assert_eq!(i.to_u64(),   w.to_u64());
                assert_eq!(i.to_usize(), w.to_usize());
                assert_eq!(i.to_i8(),    w.to_i8());
                assert_eq!(i.to_i16(),   w.to_i16());
                assert_eq!(i.to_i32(),   w.to_i32());
                assert_eq!(i.to_i64(),   w.to_i64());
                assert_eq!(i.to_isize(), w.to_isize());
                assert_eq!(i.to_f32(),   w.to_f32());
                assert_eq!(i.to_f64(),   w.to_f64());
            })+
        };
    }

    test_wrapping_to_primitive!(usize u8 u16 u32 u64 isize i8 i16 i32 i64);
}

#[test]
fn wrapping_is_toprimitive() {
    fn require_toprimitive<T: ToPrimitive>(_: &T) {}
    require_toprimitive(&Wrapping(42));
}

#[test]
fn wrapping_is_fromprimitive() {
    fn require_fromprimitive<T: FromPrimitive>(_: &T) {}
    require_fromprimitive(&Wrapping(42));
}

#[test]
fn wrapping_is_numcast() {
    fn require_numcast<T: NumCast>(_: &T) {}
    require_numcast(&Wrapping(42));
}

#[test]
fn as_primitive() {
    let x: f32 = (1.625f64).as_();
    assert_eq!(x, 1.625f32);

    let x: f32 = (3.14159265358979323846f64).as_();
    assert_eq!(x, 3.1415927f32);

    let x: u8 = (768i16).as_();
    assert_eq!(x, 0);
}

#[test]
fn float_to_integer_checks_overflow() {
    // This will overflow an i32
    let source: f64 = 1.0e+123f64;

    // Expect the overflow to be caught
    assert_eq!(cast::<f64, i32>(source), None);
}

#[test]
fn cast_to_int_checks_overflow() {
    let big_f: f64 = 1.0e123;
    let normal_f: f64 = 1.0;
    let small_f: f64 = -1.0e123;
    assert_eq!(None, cast::<f64, isize>(big_f));
    assert_eq!(None, cast::<f64, i8>(big_f));
    assert_eq!(None, cast::<f64, i16>(big_f));
    assert_eq!(None, cast::<f64, i32>(big_f));
    assert_eq!(None, cast::<f64, i64>(big_f));

    assert_eq!(Some(normal_f as isize), cast::<f64, isize>(normal_f));
    assert_eq!(Some(normal_f as i8), cast::<f64, i8>(normal_f));
    assert_eq!(Some(normal_f as i16), cast::<f64, i16>(normal_f));
    assert_eq!(Some(normal_f as i32), cast::<f64, i32>(normal_f));
    assert_eq!(Some(normal_f as i64), cast::<f64, i64>(normal_f));

    assert_eq!(None, cast::<f64, isize>(small_f));
    assert_eq!(None, cast::<f64, i8>(small_f));
    assert_eq!(None, cast::<f64, i16>(small_f));
    assert_eq!(None, cast::<f64, i32>(small_f));
    assert_eq!(None, cast::<f64, i64>(small_f));
}

#[test]
fn cast_to_unsigned_int_checks_overflow() {
    let big_f: f64 = 1.0e123;
    let normal_f: f64 = 1.0;
    let small_f: f64 = -1.0e123;
    assert_eq!(None, cast::<f64, usize>(big_f));
    assert_eq!(None, cast::<f64, u8>(big_f));
    assert_eq!(None, cast::<f64, u16>(big_f));
    assert_eq!(None, cast::<f64, u32>(big_f));
    assert_eq!(None, cast::<f64, u64>(big_f));

    assert_eq!(Some(normal_f as usize), cast::<f64, usize>(normal_f));
    assert_eq!(Some(normal_f as u8), cast::<f64, u8>(normal_f));
    assert_eq!(Some(normal_f as u16), cast::<f64, u16>(normal_f));
    assert_eq!(Some(normal_f as u32), cast::<f64, u32>(normal_f));
    assert_eq!(Some(normal_f as u64), cast::<f64, u64>(normal_f));

    assert_eq!(None, cast::<f64, usize>(small_f));
    assert_eq!(None, cast::<f64, u8>(small_f));
    assert_eq!(None, cast::<f64, u16>(small_f));
    assert_eq!(None, cast::<f64, u32>(small_f));
    assert_eq!(None, cast::<f64, u64>(small_f));
}

#[test]
fn cast_float_to_int_edge_cases() {
    use core::mem::transmute;

    trait RawOffset: Sized {
        type Raw;
        fn raw_offset(self, offset: Self::Raw) -> Self;
    }
    impl RawOffset for f32 {
        type Raw = i32;
        fn raw_offset(self, offset: Self::Raw) -> Self {
            unsafe {
                let raw: Self::Raw = transmute(self);
                transmute(raw + offset)
            }
        }
    }
    impl RawOffset for f64 {
        type Raw = i64;
        fn raw_offset(self, offset: Self::Raw) -> Self {
            unsafe {
                let raw: Self::Raw = transmute(self);
                transmute(raw + offset)
            }
        }
    }

    macro_rules! test_edge {
        ($f:ident -> $($t:ident)+) => { $({
            println!("testing cast edge cases for {} -> {}", stringify!($f), stringify!($t));

            let small = if $t::MIN == 0 || size_of::<$t>() < size_of::<$f>() {
                $t::MIN as $f - 1.0
            } else {
                ($t::MIN as $f).raw_offset(1).floor()
            };
            let fmin = small.raw_offset(-1);
            println!("  testing min {}\n\tvs. {:.16}\n\tand {:.16}", $t::MIN, fmin, small);
            assert_eq!(Some($t::MIN), cast::<$f, $t>($t::MIN as $f));
            assert_eq!(Some($t::MIN), cast::<$f, $t>(fmin));
            assert_eq!(None, cast::<$f, $t>(small));

            let (max, large) = if size_of::<$t>() < size_of::<$f>() {
                ($t::MAX, $t::MAX as $f + 1.0)
            } else {
                let large = $t::MAX as $f; // rounds up!
                let max = large.raw_offset(-1) as $t; // the next smallest possible
                assert_eq!(max.count_ones(), $f::MANTISSA_DIGITS);
                (max, large)
            };
            let fmax = large.raw_offset(-1);
            println!("  testing max {}\n\tvs. {:.16}\n\tand {:.16}", max, fmax, large);
            assert_eq!(Some(max), cast::<$f, $t>(max as $f));
            assert_eq!(Some(max), cast::<$f, $t>(fmax));
            assert_eq!(None, cast::<$f, $t>(large));

            println!("  testing non-finite values");
            assert_eq!(None, cast::<$f, $t>($f::NAN));
            assert_eq!(None, cast::<$f, $t>($f::INFINITY));
            assert_eq!(None, cast::<$f, $t>($f::NEG_INFINITY));
        })+}
    }

    test_edge!(f32 -> isize i8 i16 i32 i64);
    test_edge!(f32 -> usize u8 u16 u32 u64);
    test_edge!(f64 -> isize i8 i16 i32 i64);
    test_edge!(f64 -> usize u8 u16 u32 u64);
}
