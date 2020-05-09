use std::fmt::{Display, Debug};

#[derive(Copy, Clone, PartialEq, Hash, Debug)]
#[allow(dead_code)]
pub enum StackError {
    OutOfBounds
}

pub trait Stack<T: Sized + Clone+ Eq + Display + Debug>: Clone + Eq + Debug +
    Display + IntoIterator {
    fn new() -> Self;
    fn push(&mut self, elem: T) -> Result<(), StackError>;
    fn pop(&mut self) -> Result<T, StackError>;
    fn peek(&self) -> Result<&T, StackError>;
    fn depth(&self) -> Result<usize, StackError>;
    fn clear(&mut self) -> Result<(), StackError>;
}

