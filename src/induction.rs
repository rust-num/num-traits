use core::ops::Add;

use crate::{One, Zero};

pub trait Induction: Zero + One {
    fn nth<const N: usize>() -> Self;
}

impl<T> Induction for T
where
    T: Zero + One + Add<Self>,
{
    // I wasn't able to implement this with const-ness at the time of initially writing this.
    // Please feel free to update the code to something else.
    //
    // I was also tempted to write this recursively as it looks prettier, but rust doesn't optimize
    // tail calls in debug mode. This means we overflow the stack really quickly in debug mode. So
    // anyone using this trait and compiling in debug would potentially run into panics due to
    // overflow
    fn nth<const N: usize>() -> Self {
        let mut res = T::zero();
        for _ in 0..N {
            res = res + T::one();
        }
        res
    }
}

#[cfg(test)]
trait TestRequirements: Induction + std::fmt::Debug + PartialEq + crate::NumCast {}
#[cfg(test)]
impl<T: Induction + std::fmt::Debug + PartialEq + crate::NumCast> TestRequirements for T {}

#[cfg(test)]
fn assert_eq_nth_n<S: TestRequirements, const N: usize>() {
    assert_eq!(
        <S as crate::NumCast>::from(N)
            .unwrap_or_else(|| panic!("Couldn't convert {N} to {}", std::any::type_name::<S>())),
        S::nth::<N>()
    );
}

#[cfg(test)]
fn nth_t<T: TestRequirements>() {
    assert_eq_nth_n::<T, 0>();
    assert_eq_nth_n::<T, 1>();
    assert_eq_nth_n::<T, 2>();
    // biggest number that's safe for all primitive types
    assert_eq_nth_n::<T, 127>();
}

#[test]
fn basic_nth_all_types() {
    nth_t::<i8>();
    nth_t::<i16>();
    nth_t::<i32>();
    nth_t::<i64>();
    nth_t::<isize>();

    nth_t::<u8>();
    nth_t::<u16>();
    nth_t::<u32>();
    nth_t::<u64>();
    nth_t::<usize>();

    nth_t::<f32>();
    nth_t::<f64>();
}

#[test]
#[should_panic]
fn nth_overflow() {
    i8::nth::<128>();
}
