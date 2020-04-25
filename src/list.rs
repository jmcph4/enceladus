use std::ops::{Index, IndexMut};
use std::fmt;
use std::fmt::{Debug, Display};
use std::error::Error;
use std::hash::Hash;

#[derive(Copy, Clone, PartialEq, Hash, Debug)]
pub enum ListError {
    OutOfBounds
}

impl Eq for ListError {}

impl fmt::Display for ListError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ListError::OutOfBounds => {
                write!(f, "Requested index is out of bounds")?
            }
        }
    
        Ok(())
    }
}

pub trait List<T: Sized + Clone + Eq + Display + Debug>: Clone + Eq + Debug +
    Display + IntoIterator + Index<usize> + IndexMut<usize>  {
    fn new() -> Self;
    fn get(&self, pos: usize) -> Result<&T, ListError>;
    fn get_mut(&mut self, pos: usize) -> Result<&mut T, ListError>;
    fn set(&mut self, pos: usize, elem: T) -> Result<(), ListError>;
    fn insert(&mut self, pos: usize, elem: T) -> Result<(), ListError>;
    fn remove(&mut self, pos: usize) -> Result<T, ListError>;
    fn length(&self) -> Result<usize, ListError>;
    fn append(&mut self, elem: T) -> Result<(), ListError>;
    fn swap(&mut self, a: usize, b: usize) -> Result<(), ListError>;
    fn contains(&self, elem: T) -> Result<bool, ListError>;
    fn find_all(&self, elem: T) -> Result<Option<Vec<usize>>, ListError>;
    fn find(&self, elem: T) -> Result<Option<usize>, ListError>;
    fn count(&self, elem: T) -> Result<usize, ListError>;
    fn clear(&mut self) -> Result<(), ListError>;
}

