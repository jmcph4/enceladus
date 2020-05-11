use std::collections::BinaryHeap;
use std::collections::binary_heap::IntoIter;
use std::fmt::{Debug, Display};

use crate::error::EnceladusError;
use crate::priority_queue::PriorityQueue;

#[derive(Clone, Debug)]
pub struct Heap<T> {
    elems: BinaryHeap<T>
}

impl<T: Eq> PartialEq for Heap<T> {
    fn eq(&self, other: &Self) -> bool {
        let mut local_elems: Vec<&T> = Vec::new();
        let mut other_elems: Vec<&T> = Vec::new();
        
        for elem in self.elems.iter() {
            local_elems.push(elem);
        }

        for elem in other.elems.iter() {
            other_elems.push(elem);
        }

        if local_elems.len() != other_elems.len() {
            return false;
        }

        for i in 0..local_elems.len() {
            if *local_elems[i] != *other_elems[i] {
                return false;
            }
        }

        return true;
    }
}

impl<T> Eq for Heap<T> where T: Eq {}

impl<T> IntoIterator for Heap<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.elems.into_iter()
    }
}

impl<T: Sized + Eq + Clone + Ord + Display + Debug> PriorityQueue<T> for
    Heap<T> {
    fn new() -> Self {
        Heap {
            elems: BinaryHeap::new()
        }
    }

    fn push(&mut self, elem: T) -> Result<(), EnceladusError> {
        self.elems.push(elem);
        Ok(())
    }

    fn pop(&mut self) -> Result<T, EnceladusError> {
        if self.elems.len() == 0 { /* bounds check */
            return Err(EnceladusError::OutOfBounds);
        }

        match self.elems.pop() {
            Some(elem) => Ok(elem),
            None => Err(EnceladusError::OutOfBounds)
        }
    }

    fn peek(&self) -> Result<&T, EnceladusError> {
        if self.elems.len() == 0 { /* bounds check */
            return Err(EnceladusError::OutOfBounds);
        }

        match self.elems.peek() {
            Some(elem) => Ok(&elem),
            None => Err(EnceladusError::OutOfBounds)
        }
    }

    fn find(&self, elem: T) -> Result<Option<usize>, EnceladusError> {
        let mut curr_pos: usize = 0;

        for local_elem in self.elems.iter() {
            if *local_elem == elem {
                return Ok(Some(curr_pos));
            }

            curr_pos += 1;
        }

        Ok(None)
    }

    fn length(&self) -> Result<usize, EnceladusError> {
        Ok(self.elems.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_normal() -> Result<(), EnceladusError> {
        let actual_priority_queue: Heap<u64> = Heap::new();
        let expected_priority_queue: Heap<u64> = Heap {
            elems: BinaryHeap::new()
        };

        assert_eq!(actual_priority_queue, expected_priority_queue);
        Ok(())
    }

    #[test]
    fn test_push_normal1() -> Result<(), EnceladusError> {
        let mut actual_priority_queue: Heap<u64> = Heap::new();

        for i in 1..10 {
            actual_priority_queue.push(i)?;
        }

        assert_eq!(actual_priority_queue.pop()?, 9);
        assert_eq!(actual_priority_queue.pop()?, 8);
        assert_eq!(actual_priority_queue.pop()?, 7);
        assert_eq!(actual_priority_queue.pop()?, 6);
        assert_eq!(actual_priority_queue.pop()?, 5);
        assert_eq!(actual_priority_queue.pop()?, 4);
        assert_eq!(actual_priority_queue.pop()?, 3);
        assert_eq!(actual_priority_queue.pop()?, 2);
        assert_eq!(actual_priority_queue.pop()?, 1);

        Ok(())
    }
}

