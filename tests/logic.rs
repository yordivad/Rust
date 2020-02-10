use parser::logic::{and, xor};

#[test]
fn and_test() {
    assert_eq!(and(true, true), true);
    assert_eq!(and(true, false), false);
}

#[test]
fn or_test() {
    assert_eq!(xor(true, true), true);
    assert_eq!(xor(true, false), true);
    assert_eq!(xor(false, false), false);
}
