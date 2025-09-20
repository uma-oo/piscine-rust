use std::ops::{ Add, Sub };
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ThreeDVector<T> {
    pub i: T,
    pub j: T,
    pub k: T,
}


 impl<T: std::ops::Add<Output = T>> Add for ThreeDVector<T>{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self { i: self.i + other.i, j: self.j + other.j, k: self.k + other.k }
    }
}

impl<T: std::ops::Sub<Output = T>> Sub for ThreeDVector<T>{
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self { i: self.i - other.i, j: self.j - other.j, k: self.k - other.k }
    }
}
