use std::ops::{Add, AddAssign, Mul};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vec2<T: Add<Output = T> + Mul<Output = T> + AddAssign + Copy> {
    pub x: T,
    pub y: T,
}

impl<T: Add<Output = T> + Mul<Output = T> + AddAssign + Copy> Add for Vec2<T> {
    type Output = Vec2<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x.add(rhs.x),
            y: self.y.add(rhs.y),
        }
    }
}

impl<T: Add<Output = T> + Mul<Output = T> + AddAssign + Copy> Mul for Vec2<T> {
    type Output = Vec2<T>;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x.mul(rhs.x),
            y: self.y.mul(rhs.y),
        }
    }
}

impl<T: Add<Output = T> + Mul<Output = T> + AddAssign + Copy> Mul<T> for Vec2<T> {
    type Output = Vec2<T>;
    fn mul(self, rhs: T) -> Self::Output {
        Self::Output {
            x: self.x.mul(rhs),
            y: self.y.mul(rhs),
        }
    }
}

impl<T: Add<Output = T> + Mul<Output = T> + AddAssign + Copy> AddAssign for Vec2<T> {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.add(rhs);
    }
}

impl<T: Add<Output = T> + Mul<Output = T> + AddAssign + Copy> From<(T, T)> for Vec2<T> {
    fn from(value: (T, T)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}
