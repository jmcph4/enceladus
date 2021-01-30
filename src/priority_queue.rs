use std::fmt::{Debug, Display};

use crate::error::EnceladusError;

pub trait PriorityQueue<T: Sized + Eq + Clone + PartialOrd + Display + Debug>:
    Eq + Clone + IntoIterator
{
    fn new() -> Self;
    fn push(&mut self, elem: T) -> Result<(), EnceladusError>;
    fn pop(&mut self) -> Result<T, EnceladusError>;
    fn peek(&self) -> Result<&T, EnceladusError>;
    fn find(&self, elem: T) -> Result<Option<usize>, EnceladusError>;
    fn length(&self) -> Result<usize, EnceladusError>;
}
