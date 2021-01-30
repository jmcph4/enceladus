use std::fmt::{Debug, Display};

use crate::error::EnceladusError;

pub trait Set<T: Sized + Clone + Eq + Display + Debug>:
    Clone + Eq + Debug + Display + IntoIterator
{
    fn new() -> Self;
    fn add(&mut self, elem: T) -> Result<(), EnceladusError>;
    fn remove(&mut self, elem: T) -> Result<(), EnceladusError>;
    fn contains(&self, elem: T) -> Result<bool, EnceladusError>;
    fn size(&self) -> Result<usize, EnceladusError>;
    fn clear(&mut self) -> Result<(), EnceladusError>;
}
