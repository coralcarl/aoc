use std::{fmt::Debug, str::FromStr};

pub fn read_number_grid<T: FromStr>(input: &str, token: &str) -> Vec<Vec<T>>
where
    <T as FromStr>::Err: Debug,
{
    let mut numbers = Vec::new();
    for line in input.lines() {
        numbers.push(
            line.split(token)
                .map(|x| x.parse::<T>().expect("Unable to parse"))
                .collect(),
        );
    }
    numbers
}

pub mod geometry {
    #![allow(dead_code)]

    use std::{
        array,
        ops::{Add, AddAssign, Index, Mul, Sub},
        slice::SliceIndex,
    };

    use num::traits::{WrappingAdd, WrappingSub};

    pub type Point<T> = GenericPoint<T, 2>;

    impl<T> Point<T>
    where
        T: num::traits::One + num::traits::Zero + Copy + Add + WrappingSub,
    {
        pub fn neighbors(self) -> [Self; 4] {
            [
                self + Point::new([T::one(), T::zero()]),
                self + Point::new([T::zero(), T::one()]),
                self.wrapping_sub(&Point::new([T::one(), T::zero()])),
                self.wrapping_sub(&Point::new([T::zero(), T::one()])),
            ]
        }
    }

    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
    pub struct GenericPoint<T, const N: usize> {
        data: [T; N],
    }

    impl<T, const N: usize> GenericPoint<T, N> {
        pub fn new(data: [T; N]) -> Self {
            Self { data }
        }
    }

    impl<T, const N: usize> GenericPoint<T, N>
    where
        T: WrappingAdd + Copy,
    {
        pub fn wrapping_add(self, rhs: &Self) -> Self {
            Self::new(array::from_fn(|i| self.data[i].wrapping_add(&rhs.data[i])))
        }
    }

    impl<T, const N: usize> GenericPoint<T, N>
    where
        T: WrappingSub + Copy,
    {
        pub fn wrapping_sub(self, rhs: &Self) -> Self {
            Self::new(array::from_fn(|i| self.data[i].wrapping_sub(&rhs.data[i])))
        }
    }

    impl<T, const N: usize, Idx> Index<Idx> for GenericPoint<T, N>
    where
        Idx: SliceIndex<[T], Output = T>,
    {
        type Output = T;

        fn index(&self, index: Idx) -> &Self::Output {
            self.data.index(index)
        }
    }

    impl<T: Default, const N: usize> Default for GenericPoint<T, N> {
        fn default() -> Self {
            Self {
                data: std::array::from_fn(|_| T::default()),
            }
        }
    }

    impl<T, const N: usize> Add for GenericPoint<T, N>
    where
        T: Add<Output = T> + Copy,
    {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            Self::new(array::from_fn(|i| self.data[i] + rhs.data[i]))
        }
    }

    impl<T, const N: usize> AddAssign for GenericPoint<T, N>
    where
        T: AddAssign + Copy,
    {
        fn add_assign(&mut self, rhs: Self) {
            for i in 0..self.data.len() {
                self.data[i] += rhs.data[i];
            }
        }
    }

    impl<T, const N: usize> Sub for GenericPoint<T, N>
    where
        T: Sub<Output = T> + Copy,
    {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self::Output {
            Self::new(array::from_fn(|i| self.data[i] - rhs.data[i]))
        }
    }

    impl<T, const N: usize> Mul<T> for GenericPoint<T, N>
    where
        T: Mul<Output = T> + Copy,
    {
        type Output = Self;

        fn mul(self, rhs: T) -> Self::Output {
            Self::new(array::from_fn(|i| self.data[i] * rhs))
        }
    }
}
