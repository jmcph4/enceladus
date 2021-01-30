use crate::error::EnceladusError;

pub trait Queue<T: Sized + Eq + Clone>: Eq + Clone {
    fn new() -> Self;
    fn push(&mut self, elem: T) -> Result<(), EnceladusError>;
    fn pop(&mut self) -> Result<T, EnceladusError>;
    fn peek(&self) -> Result<&T, EnceladusError>;
    fn length(&self) -> Result<usize, EnceladusError>;
}
