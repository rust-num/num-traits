#[test]
fn test_if_num_impl_debug() {
    use num_traits::Num;
    use std::ops::Add;
    pub struct LengthNArrayOfTypeT<const N: usize, T: Num>([T; N]);
    impl<const N: usize, T: Num + Clone> Add for LengthNArrayOfTypeT<N, T> {
        type Output = Self;
        fn add(self, rhs: Self) -> Self {
            LengthNArrayOfTypeT(
                self.0
                    .iter()
                    .zip(rhs.0.iter())
                    .map(|(a, b)| a.clone() + b.clone())
                    .collect::<Vec<T>>()
                    .try_into()
                    .unwrap(),
            )
        }
    }
}
