extern crate thiserror;

use std::fmt;

use thiserror::Error;

#[derive(Copy, Clone, PartialEq, Hash, Debug, Error)]
pub enum EnceladusError {
    OutOfBounds,
    KeyNotFound
}

impl Eq for EnceladusError {}

impl fmt::Display for EnceladusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EnceladusError::OutOfBounds => {
                write!(f, "Requested index is out of bounds")?
            },
            EnceladusError::KeyNotFound => {
                write!(f, "Requested key does not exist")?
            }
        }
    
        Ok(())
    }
}

