use core::sync::atomic::*;

pub trait Atomic: Sized {
    type NonAtomicType: Copy;

    fn new(value: Self::NonAtomicType) -> Self;
    fn load(&self, order: Ordering) -> Self::NonAtomicType;
    fn store(&self, value: Self::NonAtomicType, order: Ordering);
    fn get_mut(&mut self) -> &mut Self::NonAtomicType;
    fn into_inner(self) -> Self::NonAtomicType;

    #[cfg(feature="atomic_from_mut")]
    fn get_mut_slice(this: &mut [Self]) -> &mut [Self::NonAtomicType];
    #[cfg(feature="atomic_from_mut")]
    fn from_mut_slice(this: &mut [Self::NonAtomicType]) -> &mut [Self];

    fn compare_exchange(
        &self,
        current: Self::NonAtomicType,
        new: Self::NonAtomicType,
        success: Ordering,
        failure: Ordering,
    ) -> Result<Self::NonAtomicType, Self::NonAtomicType>;

    fn compare_exchange_weak(
        &self,
        current: Self::NonAtomicType,
        new: Self::NonAtomicType,
        success: Ordering,
        failure: Ordering,
    ) -> Result<Self::NonAtomicType, Self::NonAtomicType>;

    fn swap(
        &self,
        new: Self::NonAtomicType,
        order: Ordering,
    ) -> Self::NonAtomicType;

    fn fetch_add(&self, value: Self::NonAtomicType, order: Ordering) -> Self::NonAtomicType;
    fn fetch_saturating_add(&self, value: Self::NonAtomicType, order: Ordering) -> Self::NonAtomicType;
    fn fetch_and(&self, value: Self::NonAtomicType, order: Ordering) -> Self::NonAtomicType;
    fn fetch_max(&self, value: Self::NonAtomicType, order: Ordering) -> Self::NonAtomicType;
    fn fetch_min(&self, value: Self::NonAtomicType, order: Ordering) -> Self::NonAtomicType;
    fn fetch_nand(&self, value: Self::NonAtomicType, order: Ordering) -> Self::NonAtomicType;
    fn fetch_or(&self, value: Self::NonAtomicType, order: Ordering) -> Self::NonAtomicType;
    fn fetch_sub(&self, value: Self::NonAtomicType, order: Ordering) -> Self::NonAtomicType;
    fn fetch_xor(&self, value: Self::NonAtomicType, order: Ordering) -> Self::NonAtomicType;

    fn fetch_update<F>(
        &self, 
        set_order: Ordering, 
        fetch_order: Ordering,
        f: F,
    ) -> Result<Self::NonAtomicType, Self::NonAtomicType>
    where
        F: FnMut(Self::NonAtomicType) -> Option<Self::NonAtomicType>;
}

/// While this trait doesn't do much, it allows to use the dual specification.
/// The impl automatically mirrors the [`Atomic`] trait so
/// 
/// E.g.:
/// ```ignore
/// fn to_atomic<T: IntoAtomic>(value: T) -> T::AtomicType {
///     <T::AtomicType>::new(value)
/// }
/// ```
/// instead of
/// ```ignore
/// fn to_atomic<R: Atomic>(value: R::NonAtomicType) -> R {
///     <R>::new(value)
/// }
/// ```
pub trait IntoAtomic: Sized + Send + Sync {
    type AtomicType: Atomic<NonAtomicType=Self>;

    #[cfg(feature="atomic_from_mut")]
    fn get_mut_slice(this: &mut [Self::AtomicType]) -> &mut [Self];
    
    #[cfg(feature="atomic_from_mut")]
    fn from_mut_slice(this: &mut [Self]) -> &mut [Self::AtomicType];

}

macro_rules! impl_atomic_trait {
    ($($non_atomic:ty, $atomic:ty,)*) => {$(

impl IntoAtomic for $non_atomic {
    type AtomicType = $atomic;

    #[cfg(feature="atomic_from_mut")]
    #[inline]
    fn get_mut_slice(this: &mut [Self::AtomicType]) -> &mut [Self]{
        <$atomic>::get_mut_slice(this)
    }

    #[cfg(feature="atomic_from_mut")]
    #[inline]
    fn from_mut_slice(this: &mut [Self]) -> &mut [Self::AtomicType]{
        <$atomic>::from_mut_slice(this)
    }
}

impl Atomic for $atomic {
    type NonAtomicType = $non_atomic;

    #[inline]
    fn new(value: Self::NonAtomicType) -> Self {
        <$atomic>::new(value)
    }

    #[inline]
    fn load(&self, order: Ordering) -> Self::NonAtomicType {
        <$atomic>::load(self, order)
    }

    #[inline]
    fn store(&self, value: Self::NonAtomicType, order: Ordering) {
        <$atomic>::store(self, value, order)
    }

    #[inline]
    fn get_mut(&mut self) -> &mut Self::NonAtomicType {
        <$atomic>::get_mut(self)
    }

    #[inline]
    fn into_inner(self) -> Self::NonAtomicType {
        <$atomic>::into_inner(self)
    }

    #[cfg(feature="atomic_from_mut")]
    #[inline]
    fn get_mut_slice(this: &mut [Self]) -> &mut [Self::NonAtomicType]{
        <$atomic>::get_mut_slice(this)
    }

    #[cfg(feature="atomic_from_mut")]
    #[inline]
    fn from_mut_slice(this: &mut [Self::NonAtomicType]) -> &mut [Self]{
        <$atomic>::from_mut_slice(this)
    }

    #[inline]
    fn compare_exchange(
        &self,
        current: Self::NonAtomicType,
        new: Self::NonAtomicType,
        success: Ordering,
        failure: Ordering,
    ) -> Result<Self::NonAtomicType, Self::NonAtomicType> {
        <$atomic>::compare_exchange(
            self,
            current,
            new,
            success,
            failure,
        )
    }


    #[inline]    
    fn compare_exchange_weak(
        &self,
        current: Self::NonAtomicType,
        new: Self::NonAtomicType,
        success: Ordering,
        failure: Ordering,
    ) -> Result<Self::NonAtomicType, Self::NonAtomicType>{
        <$atomic>::compare_exchange_weak(
            self,
            current,
            new,
            success,
            failure,
        )
    }

    #[inline]
    fn swap(
        &self,
        new: Self::NonAtomicType,
        order: Ordering,
    ) -> Self::NonAtomicType{
        <$atomic>::swap(
            self,
            new,
            order,
        )
    }

    #[inline]
    fn fetch_add(&self, value: Self::NonAtomicType, order: Ordering) -> Self::NonAtomicType{
        <$atomic>::fetch_add(self, value, order)
    }
    
    #[inline]
    fn fetch_saturating_add(&self, value: Self::NonAtomicType, order: Ordering) -> Self::NonAtomicType{
        let mut base = <$atomic>::load(self, order);
        loop {
            let new = base.saturating_add(value);
            let res = <$atomic>::compare_exchange_weak(
                self,
                base,
                new,
                order,
                order,
            );
            match res {
                Ok(val) => {return val},
                Err(val) => {
                    base = val;
                }
            }
        }
    }

    #[inline]
    fn fetch_and(&self, value: Self::NonAtomicType, order: Ordering) -> Self::NonAtomicType{
        <$atomic>::fetch_and(self, value, order)
    }
    #[inline]
    fn fetch_max(&self, value: Self::NonAtomicType, order: Ordering) -> Self::NonAtomicType{
        <$atomic>::fetch_max(self, value, order)
    }
    #[inline]
    fn fetch_min(&self, value: Self::NonAtomicType, order: Ordering) -> Self::NonAtomicType{
        <$atomic>::fetch_min(self, value, order)
    }
    #[inline]
    fn fetch_nand(&self, value: Self::NonAtomicType, order: Ordering) -> Self::NonAtomicType{
        <$atomic>::fetch_nand(self, value, order)
    }
    #[inline]
    fn fetch_or(&self, value: Self::NonAtomicType, order: Ordering) -> Self::NonAtomicType{
        <$atomic>::fetch_or(self, value, order)
    }
    #[inline]
    fn fetch_sub(&self, value: Self::NonAtomicType, order: Ordering) -> Self::NonAtomicType{
        <$atomic>::fetch_sub(self, value, order)
    }
    #[inline]
    fn fetch_xor(&self, value: Self::NonAtomicType, order: Ordering) -> Self::NonAtomicType{
        <$atomic>::fetch_xor(self, value, order)
    }

    #[inline]
    fn fetch_update<F>(
        &self, 
        set_order: Ordering, 
        fetch_order: Ordering,
        f: F,
    ) -> Result<Self::NonAtomicType, Self::NonAtomicType>
    where
        F: FnMut(Self::NonAtomicType) -> Option<Self::NonAtomicType> {
        <$atomic>::fetch_update(self, set_order, fetch_order, f)
    }
}

)*};
}

impl_atomic_trait!{
    u8,  AtomicU8,
    u16, AtomicU16,
    u32, AtomicU32,
    u64, AtomicU64,
    usize, AtomicUsize,
    i8,  AtomicI8,
    i16, AtomicI16,
    i32, AtomicI32,
    i64, AtomicI64,
    isize, AtomicIsize,
}