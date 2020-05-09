use std::fmt::{Debug, Display};

use crate::error::EnceladusError;

pub trait Stack<T: Sized + Clone+ Eq + Display + Debug>: Clone + Eq + Debug +
    Display + IntoIterator {
    fn new() -> Self;
    fn push(&mut self, elem: T) -> Result<(), EnceladusError>;
    fn pop(&mut self) -> Result<T, EnceladusError>;
    fn peek(&self) -> Result<&T, EnceladusError>;
    fn depth(&self) -> Result<usize, EnceladusError>;
    fn clear(&mut self) -> Result<(), EnceladusError>;
}

