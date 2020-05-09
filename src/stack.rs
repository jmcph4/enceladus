extern crate thiserror;

use std::fmt;
use std::fmt::{Debug, Display};
use std::hash::Hash;

use thiserror::Error;

#[derive(Copy, Clone, PartialEq, Hash, Debug, Error)]
pub enum StackError {
    OutOfBounds
}

impl Eq for StackError {}

impl fmt::Display for StackError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StackError::OutOfBounds => {
                write!(f, "Insufficient elements")?
            }
        }
    
        Ok(())
    }
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

