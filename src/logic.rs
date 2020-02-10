pub fn and(a: bool, b: bool) -> bool {
    match (a, b) {
        (true, true) => true,
        _ => false,
    }
}

pub fn xor(a: bool, b: bool) -> bool {
    match (a, b) {
        (false, false) => false,
        _ => true,
    }
}
