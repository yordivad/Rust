use crate::cage::Cage::Element;

pub enum Cage<T> {
    Empty,
    Element(T),
}

impl<T> Cage<T> {
    pub fn new(value: T) -> Cage<T> {
        Element(value)
    }
}
