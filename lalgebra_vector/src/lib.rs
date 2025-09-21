use std::ops::Add;
use std::ops::Mul;
use std::ops::AddAssign;
pub trait Scalar: Sized + Add<Output = Self> + Mul<Output = Self> + Clone + AddAssign {
    type Item;
    fn zero() -> Self::Item;
    fn one() -> Self::Item;
}

impl Scalar for i32 {
    type Item = i32;
    fn zero() -> Self::Item {
        0
    }
    fn one() -> Self::Item {
        1
    }
}

impl Scalar for i64 {
    type Item = i64;
    fn zero() -> Self::Item {
        0
    }
    fn one() -> Self::Item {
        1
    }
}

impl Scalar for u32 {
    type Item = u32;
    fn zero() -> Self::Item {
        0
    }
    fn one() -> Self::Item {
        1
    }
}

impl Scalar for u64 {
    type Item = u64;
    fn zero() -> Self::Item {
        0
    }
    fn one() -> Self::Item {
        1
    }
}

impl Scalar for f64 {
    type Item = f64;
    fn zero() -> Self::Item {
        0.0
    }
    fn one() -> Self::Item {
        1.0
    }
}

impl Scalar for f32 {
    type Item = f64;
    fn zero() -> f64 {
        0.0
    }
    fn one() -> f64 {
        1.0
    }
}

// impl Add for Scalar {

//     fn add(self, other: Self) -> Self::Output {
//         self + other
//     }
// }

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Vector<T: Scalar>(pub Vec<T>);

impl<T: Scalar> Add for Vector<T> {
    type Output = Option<Vector<T>>;
    fn add(self, other: Vector<T>) -> Self::Output {
        if self.0.len() != other.0.len() {
            return None;
        }

        let mut result = Vec::new();
        for i in 0..self.0.len() {
            let sum = self.0[i].clone() + other.0[i].clone();
            result.push(sum);
        }
        Some(Vector(result))
    }
}

impl<T: Scalar<Item = T>> Vector<T> {
    pub fn new(vector: Vec<T>) -> Self {
        Self(vector)
    }

    pub fn dot(&self, other: &Self) -> Option<T> {
        if self.0.len() != other.0.len() {
            return None;
        }
        let mut sum = T::zero();

        for i in 0..self.0.len() {
            sum += self.0[i].clone() * other.0[i].clone();
        }

        Some(sum)
    }
}
