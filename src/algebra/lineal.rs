use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone)]
pub struct Vector {
    data: Vec<f64>,
}

impl Vector {
    pub fn dim(&self) -> usize {
        self.data.len()
    }

    pub fn sum(&self) -> f64 {
        let mut x = 0.0;
        for i in self.data.iter() {
            x = x + *i
        }
        return x;
    }

    pub fn distance(&self) -> f64 {
        let mut r = 0.0;
        for (x, y) in self.data.iter().zip(&self.data) {
            r += x * y;
        }
        r.sqrt()
    }

    pub fn cross(&self, rhs: &Vector) -> Vector {
        unimplemented!()
    }
}

impl From<Vec<f64>> for Vector {
    fn from(vector: Vec<f64>) -> Self {
        Vector { data: vector }
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, other: Self) -> Self::Output {
        let mut r: Vec<f64> = Vec::new();
        for (x, y) in self.data.iter().zip(other.data) {
            r.push(x + y)
        }
        Vector::from(r)
    }
}

impl Mul<f64> for &Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Self::Output {
        let mut r: Vec<f64> = Vec::new();

        for x in self.data.iter() {
            r.push(x * rhs)
        }

        Vector::from(r)
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Self::Output {
        rhs * &self
    }
}

impl Mul<&Vector> for f64 {
    type Output = Vector;

    fn mul(self, rhs: &Vector) -> Self::Output {
        rhs * self
    }
}

impl Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        self * rhs
    }
}

impl Mul for Vector {
    type Output = f64;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut r = 0.0;
        for (x, y) in self.data.iter().zip(&rhs.data) {
            r += x * y
        }
        r
    }
}

impl Sub for Vector {
    type Output = Vector;
    fn sub(self, rhs: Self) -> Self::Output {
        self + (rhs * -1.0)
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        for (x, y) in self.data.iter().zip(&other.data) {
            if x != y {
                return false;
            }
        }
        true
    }
}
