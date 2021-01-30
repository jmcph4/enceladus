use std::fmt::{Debug, Display};
use std::ops::{Index, IndexMut};

use crate::error::EnceladusError;

pub trait List<T: Sized + Clone + Eq + Display + Debug>:
    Clone + Eq + Debug + Display + IntoIterator + Index<usize> + IndexMut<usize>
{
    fn new() -> Self;
    fn get(&self, pos: usize) -> Result<&T, EnceladusError>;
    fn get_mut(&mut self, pos: usize) -> Result<&mut T, EnceladusError>;
    fn set(&mut self, pos: usize, elem: T) -> Result<(), EnceladusError>;
    fn insert(&mut self, pos: usize, elem: T) -> Result<(), EnceladusError>;
    fn remove(&mut self, pos: usize) -> Result<T, EnceladusError>;
    fn length(&self) -> Result<usize, EnceladusError>;
    fn append(&mut self, elem: T) -> Result<(), EnceladusError>;
    fn swap(&mut self, a: usize, b: usize) -> Result<(), EnceladusError>;
    fn contains(&self, elem: T) -> Result<bool, EnceladusError>;
    fn find_all(&self, elem: T) -> Result<Option<Vec<usize>>, EnceladusError>;
    fn find(&self, elem: T) -> Result<Option<usize>, EnceladusError>;
    fn count(&self, elem: T) -> Result<usize, EnceladusError>;
    fn clear(&mut self) -> Result<(), EnceladusError>;
}
