use std::fmt::{Debug, Display, Error, Formatter};
use std::ops::Add;

#[derive(Debug)]
pub enum Tensor {
    Rank0(f64),
    Rank1(Vec<f64>),
    Rank2(Vec<Vec<f64>>),
    Rank3(Vec<Vec<Vec<f64>>>),
}

pub trait Component<T> {
    fn unwrap(&self) -> T;
}

impl Component<f64> for Tensor {
    fn unwrap(&self) -> f64 {
        match self {
            Tensor::Rank0(v) => *v,
            _ => 0.0,
        }
    }
}

impl Component<Vec<f64>> for Tensor {
    fn unwrap(&self) -> Vec<f64> {
        match self {
            Tensor::Rank1(v) => v.to_vec(),
            _ => vec![],
        }
    }
}

impl From<f64> for Tensor {
    fn from(component: f64) -> Self {
        Tensor::Rank0(component)
    }
}

impl From<Vec<f64>> for Tensor {
    fn from(component: Vec<f64>) -> Self {
        Tensor::Rank1(component)
    }
}

impl Display for Tensor {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Tensor::Rank0(a) => write!(f, "tensor rank: 0, dim: 0, value: {}", a),
            Tensor::Rank1(a) => write!(f, "tensor rank: 1, dim: {}", a.len()),
            Tensor::Rank2(a) => write!(f, "tensor rank: 2, dim[1]:{}", a.len()),
            Tensor::Rank3(a) => write!(f, "tensor rank: 3, dim[1]: {}", a.len()),
        }
    }
}

trait Vector<T> {
    fn sum<'a>(&self, other: &Vec<T>) -> Result<Vec<T>, &'a str>;
}

impl<T> Vector<T> for Vec<T>
where
    T: Add<T, Output = T> + Copy,
{
    fn sum<'a>(&self, other: &Vec<T>) -> Result<Vec<T>, &'a str> {
        if self.len() != other.len() {
            return Err("invalid dimension");
        }

        let mut result: Vec<T> = Vec::new();
        for i in 0..self.len() {
            result.push(self[i] + other[i]);
        }

        Ok(result)
    }
}

impl Add for Tensor {
    type Output = Tensor;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Tensor::Rank0(a) => match rhs {
                Tensor::Rank0(b) => Tensor::from(a + b),
                _ => panic!("the ranks are not equal"),
            },
            Tensor::Rank1(a) => match rhs {
                Tensor::Rank1(b) => Tensor::from(a.sum(&b).unwrap()),
                _ => panic!("the ranks are not equal"),
            },
            Tensor::Rank2(_) => Tensor::Rank0(1.0),
            Tensor::Rank3(_) => Tensor::Rank0(1.0),
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn tensor_add() {
        let x = vec![1.0, 2.0, 3.2];
        let y = vec![1.0, 2.0, 3.1];
        let r = x.sum(&y).unwrap();
        assert_eq!(r, vec![2.0, 4.0, 6.300000000000001])
    }

    #[test]
    fn rank0_add() {
        let t1 = Tensor::from(1.0);
        let t2 = Tensor::from(2.0);
        let t3 = t1 + t2;
        assert_eq!(3.0, t3.unwrap());
    }

    #[test]
    fn rank1_add() {
        let t1 = Tensor::from(vec![1.0, 2.0]);
        let t2 = Tensor::from(vec![1.0, 2.0]);
        let t3 = t1 + t2;
        let r: Vec<f64> = t3.unwrap();
        assert_eq!(vec![2.0, 4.0], r);
    }

    #[test]
    fn rank0_add_rank1() {
        let t1 = Tensor::from(1.0);
        let t2 = Tensor::from(vec![1.0, 2.0]);
        let t3 = t1 + t2;
    }
}
