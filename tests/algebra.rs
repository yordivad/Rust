#[cfg(test)]
mod vector {
    use parser::algebra::lineal::Vector;

    #[test]
    fn vector_addition() {
        let a = Vector::from(vec![1.0, 2.0, 3.0]);
        let b = Vector::from(vec![1.0, 2.0]);
        let c = a + b;
        let e = Vector::from(vec![2.0, 4.0, 3.0]);
        assert_eq!(e, c)
    }

    #[test]
    fn vector_subs() {
        let a = Vector::from(vec![1.0, 2.0, 3.0]);
        let b = Vector::from(vec![3.0, 0.0, 1.0]);
        let e = Vector::from(vec![-2.0, 2.0, 2.0]);

        assert_eq!(e, a - b)
    }

    #[test]
    fn vector_distance() {
        let a = Vector::from(vec![3.0, 4.0]);
        assert_eq!(5.0, a.distance())
    }

    #[test]
    fn scalar_multiplication() {
        let a = Vector::from(vec![1.0, 2.0, 3.0]);
        let e = Vector::from(vec![2.0, 4.0, 6.0]);
        assert_eq!(e, 2.0 * &a);
        assert_eq!(e, &a * 2.0);
    }

    #[test]
    fn dot_product_vector() {
        let a = Vector::from(vec![1.0, 2.0]);
        let b = Vector::from(vec![1.0, 2.0]);

        assert_eq!(5.0, a * b);
    }
}
