use parser::electric::point::Point;

#[test]
fn add_point() {
    let p1 = Point::new(10.0, 10.0, 10.0);
    let p2 = Point::new(5.0, 5.0, 5.0);

    assert_eq!(p1 + p2, Point::new(15.0, 15.0, 15.0))
}

#[test]
fn sub_point() {
    let p1 = Point::new(10.0, 10.0, 10.0);
    let p2 = Point::new(2.0, 3.0, 5.0);
    let expect = Point::new(8.0, 7.0, 5.0);

    assert_eq!(p1 - p2, expect)
}
