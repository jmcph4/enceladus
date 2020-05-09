extern crate thiserror;

use std::fmt;

use thiserror::Error;

#[derive(Copy, Clone, PartialEq, Hash, Debug, Error)]
pub enum QueueError {
    OutOfBounds
}

impl Eq for QueueError {}

impl fmt::Display for QueueError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            QueueError::OutOfBounds => {
                write!(f, "Insufficient elements")?
            }
        }
    
        Ok(())
    }
}

pub trait Queue<T: Sized + Eq + Clone>: Eq + Clone {
    fn new() -> Self;
    fn push(&mut self, elem: T) -> Result<(), QueueError>;
    fn pop(&mut self) -> Result<T, QueueError>;
    fn peek(&self) -> Result<&T, QueueError>;
    fn length(&self) -> Result<usize, QueueError>;
}

