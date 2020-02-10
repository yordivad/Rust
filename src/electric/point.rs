use std::fmt::{Display, Error, Formatter};
use std::ops::{Add, Sub};

#[derive(Debug, Default, Clone)]
pub struct Point<T> {
    x: T,
    y: T,
    z: T,
}

#[derive(Debug, Default, Clone)]
pub struct SphericalPoint<T> {
    radius: f64,
    polar: f64,
    azimuth: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T, z: T) -> Point<T> {
        Point { x, y, z }
    }
}

impl<T: Add<Output = T>> Add for Point<T> {
    type Output = Point<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<T: Sub<Output = T>> Sub for Point<T>
where
    T: Sub,
{
    type Output = Point<T>;

    fn sub(self, rhs: Point<T>) -> Self::Output {
        Point::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<T: PartialEq> PartialEq for Point<T> {
    fn eq(&self, other: &Point<T>) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl<T: Display> Display for Point<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "cartesian: x:{}, y:{}, z:{}", self.x, self.y, self.z)
    }
}
