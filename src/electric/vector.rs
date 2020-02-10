use crate::electric::point::Point;
use std::ops::{Add, Sub};

pub struct Vector<T> {
    p1: Point<T>,
    p2: Point<T>,
}

impl<T> Vector<T> {
    pub fn new(p1: Point<T>, p2: Point<T>) -> Vector<T> {
        Vector { p1, p2 }
    }
}

impl<T: Add<Output = T>> Add for Vector<T> {
    type Output = Vector<T>;

    fn add(self, rhs: Vector<T>) -> Self::Output {
        Vector::new(self.p1 + rhs.p1, self.p2 + rhs.p2)
    }
}

impl<T: Sub<Output = T>> Sub for Vector<T> {
    type Output = Vector<T>;

    fn sub(self, rhs: Vector<T>) -> Self::Output {
        Vector::new(self.p1 - rhs.p1, self.p2 - rhs.p2)
    }
}
