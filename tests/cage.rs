use parser::cage::Cage;
use parser::cage::Cage::{Element, Empty};

#[test]
fn element_test() {
    let x = Cage::new(1);

    match x {
        Element(y) => assert_eq!(y, 1),
        Empty => assert!(false),
    }
}
