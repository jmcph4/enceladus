use std::ops::{Index, IndexMut};
use std::fmt;
use std::fmt::{Debug, Display};

use crate::error::EnceladusError;
use crate::list::List;

#[derive(Clone, Debug)]
pub struct ArrayList<T> {
    elems: Vec<T>
}

impl<T> PartialEq for ArrayList<T> where T: Eq {
    fn eq(&self, other: &Self) -> bool {
        if self.elems.len() != other.elems.len() {
            return false;
        }

        for i in 0..self.elems.len() {
            if self.elems[i] != other.elems[i] {
                return false;
            }
        }

        true
    }
}

impl<T> Eq for ArrayList<T> where T: Eq {}

impl<T> IntoIterator for ArrayList<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.elems.into_iter()
    }
}

impl<T> Index<usize> for ArrayList<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.elems.get(index).unwrap()
    }
}

impl<T> IndexMut<usize> for ArrayList<T>  {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.elems.get_mut(index).unwrap()
    }
}

impl<T> Display for ArrayList<T> where T: Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;

        for elem in self.elems.iter() {
            write!(f, "{}, ", elem)?;
        }

        write!(f, "]")?;

        Ok(())
    }
}

impl<T> List<T> for ArrayList<T> where T: Sized + Clone + Eq + Display + Debug {
    fn new() -> Self {
        ArrayList {
            elems: Vec::new()
        }
    }

    fn get(&self, pos: usize) -> Result<&T, EnceladusError> {
        if pos >= self.elems.len() { /* bounds check */
            return Err(EnceladusError::OutOfBounds);
        }

        match self.elems.get(pos) {
            Some(elem) => Ok(elem),
            None => Err(EnceladusError::OutOfBounds)
        }
    }
    
    fn get_mut(&mut self, pos: usize) -> Result<&mut T, EnceladusError> {
        if pos >= self.elems.len() { /* bounds check */
            return Err(EnceladusError::OutOfBounds);
        }

        match self.elems.get_mut(pos) {
            Some(elem) => Ok(elem),
            None => Err(EnceladusError::OutOfBounds)
        }
    }

    fn set(&mut self, pos: usize, elem: T) -> Result<(), EnceladusError> {
        if pos >= self.elems.len() { /* bounds check */
            return Err(EnceladusError::OutOfBounds);
        }

        self.elems[pos] = elem;
        Ok(())
    }

    fn insert(&mut self, pos: usize, elem: T) -> Result<(), EnceladusError> {
        if pos > self.elems.len() { /* bounds check */
            return Err(EnceladusError::OutOfBounds);
        }
        
        self.elems.insert(pos, elem);
        Ok(())
    }

    fn remove(&mut self, pos: usize) -> Result<T, EnceladusError> {
        if pos >= self.elems.len() { /* bounds check */
            return Err(EnceladusError::OutOfBounds);
        }

        Ok(self.elems.remove(pos))
    }

    fn length(&self) -> Result<usize, EnceladusError> {
        Ok(self.elems.len())
    }

    fn append(&mut self, elem: T) -> Result<(), EnceladusError> {
        self.elems.push(elem);
        Ok(())
    }

    fn swap(&mut self, a: usize, b: usize) -> Result<(), EnceladusError> {
        if a >= self.elems.len() || b >= self.elems.len() { /* bounds check */
            return Err(EnceladusError::OutOfBounds);
        }

        if a == b {
            return Ok(());
        }

        let tmp_a: T = self.elems.remove(a);
        let tmp_b: T;

        if b < a {
            tmp_b = self.elems.remove(b);
        } else {
            tmp_b = self.elems.remove(b - 1);
        }

        self.elems.insert(a, tmp_b);
        self.elems.insert(b, tmp_a);
        
        Ok(())
    }

    fn contains(&self, elem: T) -> Result<bool, EnceladusError> {
        for element in self.elems.iter() {
            if *element == elem {
                return Ok(true);
            }
        }

        Ok(false)
    }

    fn find_all(&self, elem: T) -> Result<Option<Vec<usize>>, EnceladusError> {
        let mut res: Vec<usize> = Vec::new();

        for i in 0..self.elems.len() {
            let curr_elem: T = match self.elems.get(i) {
                Some(curr) => curr.clone(),
                None => return Ok(None)
            };

            if curr_elem == elem {
                res.push(i);
            }
        }

        Ok(Some(res))
    }

    fn find(&self, elem: T) -> Result<Option<usize>, EnceladusError> {
        for i in 0..self.elems.len() {
            let curr_elem: T = match self.elems.get(i) {
                Some(curr) => curr.clone(),
                None => return Ok(None)
            };

            if curr_elem == elem {
                return Ok(Some(i));
            }
        }

        Ok(None)
    }

    fn count(&self, elem: T) -> Result<usize, EnceladusError> {
        let mut count: usize = 0;

        for i in 0..self.elems.len() {
            let curr_elem: T = match self.elems.get(i) {
                Some(curr) => curr.clone(),
                None => return Ok(count)
            };

            if curr_elem == elem {
                count += 1;
            }
        }

        Ok(count)
    }

    fn clear(&mut self) -> Result<(), EnceladusError> {
        if self.elems.is_empty() {
            return Ok(());
        }

        for _i in 0..self.elems.len() {
            self.elems.remove(0);
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new_normal() -> Result<(), EnceladusError> {
        let actual_list: ArrayList<u64> = ArrayList::new();
        let expected_list: ArrayList<u64> = ArrayList {
            elems: Vec::new()
        };

        assert_eq!(actual_list, expected_list);
        Ok(())
    }

    #[test]
    fn test_get_normal_middle() -> Result<(), EnceladusError> {
        let mut actual_list: ArrayList<u64> = ArrayList::new();
        let expected_list: ArrayList<u64> = ArrayList {
            elems: vec![1, 2, 3, 4]
        };

        actual_list.append(1)?;
        actual_list.append(2)?;
        actual_list.append(3)?;
        actual_list.append(4)?;
        
        let actual_res = actual_list.get(2);
        let expected_res = Ok(&3);

        assert_eq!(actual_list, expected_list);
        assert_eq!(actual_res, expected_res);

        Ok(())
    }

    #[test]
    fn test_get_normal_head() -> Result<(), EnceladusError> {
        let mut actual_list: ArrayList<u64> = ArrayList::new();
        let expected_list: ArrayList<u64> = ArrayList {
            elems: vec![1, 2, 3, 4]
        };

        actual_list.append(1)?;
        actual_list.append(2)?;
        actual_list.append(3)?;
        actual_list.append(4)?;
        
        let actual_res = actual_list.get(0);
        let expected_res = Ok(&1);

        assert_eq!(actual_list, expected_list);
        assert_eq!(actual_res, expected_res);

        Ok(())
    }

    #[test]
    fn test_get_normal_tail() -> Result<(), EnceladusError> {
        let mut actual_list: ArrayList<u64> = ArrayList::new();
        let expected_list: ArrayList<u64> = ArrayList {
            elems: vec![1, 2, 3, 4]
        };

        actual_list.append(1)?;
        actual_list.append(2)?;
        actual_list.append(3)?;
        actual_list.append(4)?;
        
        let actual_res = actual_list.get(3);
        let expected_res = Ok(&4);

        assert_eq!(actual_list, expected_list);
        assert_eq!(actual_res, expected_res);

        Ok(())
    }

    #[test]
    fn test_get_error_out_of_bounds() -> Result<(), EnceladusError> {
        let mut actual_list: ArrayList<u64> = ArrayList::new();
        let expected_list: ArrayList<u64> = ArrayList {
            elems: vec![1, 2, 3, 4]
        };

        actual_list.append(1)?;
        actual_list.append(2)?;
        actual_list.append(3)?;
        actual_list.append(4)?;
        
        let actual_res = actual_list.get(4);
        let expected_res = Err(EnceladusError::OutOfBounds);

        assert_eq!(actual_list, expected_list);
        assert_eq!(actual_res, expected_res);

        Ok(())
    }
   
    #[test]
    fn test_get_mut_normal_middle() -> Result<(), EnceladusError> {
        let mut actual_list: ArrayList<u64> = ArrayList::new();
        let expected_list: ArrayList<u64> = ArrayList {
            elems: vec![1, 2, 6, 4]
        };

        actual_list.append(1)?;
        actual_list.append(2)?;
        actual_list.append(3)?;
        actual_list.append(4)?;
        
        let actual_res: Result<&mut u64, EnceladusError> =
            actual_list.get_mut(2).map(|elem| {
            *elem *= 2;
            elem
        });
        let tmp: &mut u64 = &mut 6;
        let expected_res: Result<&mut u64, EnceladusError> = Ok(tmp);

        assert_eq!(actual_res, expected_res);
        assert_eq!(actual_list.clone(), expected_list);
        
        Ok(())
    }

    #[test]
    fn test_get_mut_normal_head() -> Result<(), EnceladusError> {
        let mut actual_list: ArrayList<u64> = ArrayList::new();
        let expected_list: ArrayList<u64> = ArrayList {
            elems: vec![2, 2, 3, 4]
        };

        actual_list.append(1)?;
        actual_list.append(2)?;
        actual_list.append(3)?;
        actual_list.append(4)?;
        
        let actual_res: Result<&mut u64, EnceladusError> =
            actual_list.get_mut(0).map(|elem| {
            *elem *= 2;
            elem
        });
        let tmp: &mut u64 = &mut 2;
        let expected_res: Result<&mut u64, EnceladusError> = Ok(tmp);

        assert_eq!(actual_res, expected_res);
        assert_eq!(actual_list, expected_list);
        
        Ok(())
    }

    #[test]
    fn test_get_mut_normal_tail() -> Result<(), EnceladusError> {
        let mut actual_list: ArrayList<u64> = ArrayList::new();
        let expected_list: ArrayList<u64> = ArrayList {
            elems: vec![1, 2, 3, 8]
        };

        actual_list.append(1)?;
        actual_list.append(2)?;
        actual_list.append(3)?;
        actual_list.append(4)?;
        
        let actual_res: Result<&mut u64, EnceladusError> =
            actual_list.get_mut(3).map(|elem| {
            *elem *= 2;
            elem
        });
        let tmp: &mut u64 = &mut 8;
        let expected_res: Result<&mut u64, EnceladusError> = Ok(tmp);
        
        assert_eq!(actual_res, expected_res);
        assert_eq!(actual_list, expected_list);
        
        Ok(())
    }

    #[test]
    fn test_get_mut_error_out_of_bounds() -> Result<(), EnceladusError> {
        let mut actual_list: ArrayList<u64> = ArrayList::new();
        let expected_list: ArrayList<u64> = ArrayList {
            elems: vec![1, 2, 3, 4]
        };

        actual_list.append(1)?;
        actual_list.append(2)?;
        actual_list.append(3)?;
        actual_list.append(4)?;
        
        let actual_res: Result<&mut u64, EnceladusError> = actual_list.get_mut(4);
        let expected_res: Result<&mut u64, EnceladusError> =
            Err(EnceladusError::OutOfBounds);

        assert_eq!(actual_res, expected_res);
        assert_eq!(actual_list, expected_list);
        
        Ok(())
    }

    #[test]
    fn test_insert_normal_middle() -> Result<(), EnceladusError> {
        let mut actual_list: ArrayList<u64> = ArrayList::new();
        let expected_list: ArrayList<u64> = ArrayList {
            elems: vec![3, 10, 12, 33]
        };

        actual_list.insert(0, 33)?;
        actual_list.insert(0, 12)?;
        actual_list.insert(0, 3)?;
        actual_list.insert(1, 10)?;

        assert_eq!(actual_list, expected_list);
        Ok(())
    }

    #[test]
    fn test_insert_error_out_of_bounds() -> Result<(), EnceladusError> {
        let mut actual_list: ArrayList<u64> = ArrayList::new();
        let expected_list: ArrayList<u64> = ArrayList {
            elems: Vec::new()
        };

        let actual_res = actual_list.insert(1, 33);
        let expected_res = Err(EnceladusError::OutOfBounds);

        assert_eq!(actual_res, expected_res);
        assert_eq!(actual_list, expected_list);
        Ok(())
    }

    #[test]
    fn test_remove_normal_middle() -> Result<(), EnceladusError> {
        let mut actual_list: ArrayList<u64> = ArrayList::new();
        let expected_list: ArrayList<u64> = ArrayList {
            elems: vec![33, 12, 10]
        };

        actual_list.append(33)?;
        actual_list.append(12)?;
        actual_list.append(1)?;
        actual_list.append(10)?;

        let actual_res: Result<u64, EnceladusError> = actual_list.remove(2);
        let expected_res: Result<u64, EnceladusError> = Ok(1);

        assert_eq!(actual_list, expected_list);
        assert_eq!(actual_res, expected_res);

        Ok(())
    }
    
    #[test]
    fn test_remove_normal_head() -> Result<(), EnceladusError> {
        let mut actual_list: ArrayList<u64> = ArrayList::new();
        let expected_list: ArrayList<u64> = ArrayList {
            elems: vec![12, 1, 10]
        };

        actual_list.append(33)?;
        actual_list.append(12)?;
        actual_list.append(1)?;
        actual_list.append(10)?;

        let actual_res: Result<u64, EnceladusError> = actual_list.remove(0);
        let expected_res: Result<u64, EnceladusError> = Ok(33);

        assert_eq!(actual_list, expected_list);
        assert_eq!(actual_res, expected_res);

        Ok(())
    }

    #[test]
    fn test_remove_normal_tail() -> Result<(), EnceladusError> {
        let mut actual_list: ArrayList<u64> = ArrayList::new();
        let expected_list: ArrayList<u64> = ArrayList {
            elems: vec![33, 12, 1]
        };

        actual_list.append(33)?;
        actual_list.append(12)?;
        actual_list.append(1)?;
        actual_list.append(10)?;

        let actual_res: Result<u64, EnceladusError> = actual_list.remove(3);
        let expected_res: Result<u64, EnceladusError> = Ok(10);

        assert_eq!(actual_list, expected_list);
        assert_eq!(actual_res, expected_res);

        Ok(())
    }

    #[test]
    fn test_remove_error_out_of_bounds() -> Result<(), EnceladusError> {
        let mut actual_list: ArrayList<u64> = ArrayList::new();
        let expected_list: ArrayList<u64> = ArrayList {
            elems: vec![33, 12, 1, 10]
        };

        actual_list.append(33)?;
        actual_list.append(12)?;
        actual_list.append(1)?;
        actual_list.append(10)?;

        let actual_res: Result<u64, EnceladusError> = actual_list.remove(4);
        let expected_res: Result<u64, EnceladusError> = Err(EnceladusError::OutOfBounds);

        assert_eq!(actual_list, expected_list);
        assert_eq!(actual_res, expected_res);

        Ok(())
    }
}

