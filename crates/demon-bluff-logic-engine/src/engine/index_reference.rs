use std::fmt::Display;

pub trait IndexReference: Display {
    fn new(index: usize) -> Self;
    fn clone(&self) -> Self;
    fn index(&self) -> usize;
}
