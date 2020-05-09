#[derive(Clone, Copy, PartialEq, Debug)]
#[allow(dead_code)]
pub enum QueueError {
    OutOfBounds
}

pub trait Queue<T: Sized + Eq + Clone>: Eq + Clone {
    fn new() -> Self;
    fn push(&mut self, elem: T) -> Result<(), QueueError>;
    fn pop(&mut self) -> Result<T, QueueError>;
    fn peek(&self) -> Result<&T, QueueError>;
    fn length(&self) -> Result<usize, QueueError>;
}

