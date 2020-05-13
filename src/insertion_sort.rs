use std::fmt::{Display, Debug};
use std::convert::TryInto;

use crate::error::EnceladusError;
use crate::list::List;

pub fn insertion_sort<L, T>(list: &mut L, cmp: fn(a: &T, b: &T) -> bool) ->
    Result<(), EnceladusError> where L: List<T>, T: Eq + Clone + Display + Debug {
    if list.length()? <= 1 {
        return Ok(());
    }

    let n: usize = list.length()?;

    for j in 1..n {
        let key: &T = &list.get(j)?.clone();

        /* insert list[j] into the sorted sequence list[0..j-1] */
        let mut i: isize = (j - 1).try_into().unwrap();

        while i >= 0 && cmp(key, list.get(i as usize)?) {
            let curr_elem: &T = &list.get(i as usize)?.clone();
            list.set((i + 1) as usize, curr_elem.clone())?;
            i -= 1;
        }

        list.set((i + 1) as usize, key.clone())?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::arraylist::ArrayList;

    fn cmp_leq<T: Eq + Ord>(a: &T, b: &T) -> bool {
        a <= b
    }

    #[test]
    pub fn test_insertion_sort_normal1() -> Result<(), EnceladusError> {
        let mut actual_list: ArrayList<u64> = ArrayList::new();

        actual_list.append(33)?;
        actual_list.append(12)?;
        actual_list.append(0)?;
        actual_list.append(1)?;
        actual_list.append(4)?;

        let mut expected_list: ArrayList<u64> = ArrayList::new();

        expected_list.append(0)?;
        expected_list.append(1)?;
        expected_list.append(4)?;
        expected_list.append(12)?;
        expected_list.append(33)?;

        let actual_res: Result<(), EnceladusError> = insertion_sort(&mut actual_list,
                                                           cmp_leq);
        let expected_res: Result<(), EnceladusError> = Ok(());

        assert_eq!(actual_list, expected_list);
        assert_eq!(actual_res, expected_res);

        Ok(())
    }

    #[test]
    pub fn test_insertion_sort_normal_empty() -> Result<(), EnceladusError> {
        let mut actual_list: ArrayList<u64> = ArrayList::new();
        let expected_list: ArrayList<u64> = ArrayList::new();

        let actual_res: Result<(), EnceladusError> = insertion_sort(&mut actual_list,
                                                           cmp_leq);
        let expected_res: Result<(), EnceladusError> = Ok(());

        assert_eq!(actual_list, expected_list);
        assert_eq!(actual_res, expected_res);

        Ok(())
    }

    #[test]
    pub fn test_insertion_sort_normal_single() -> Result<(), EnceladusError> {
        let mut actual_list: ArrayList<u64> = ArrayList::new();
        actual_list.append(1)?;

        let mut expected_list: ArrayList<u64> = ArrayList::new();
        expected_list.append(1)?;

        let actual_res: Result<(), EnceladusError> = insertion_sort(&mut actual_list,
                                                           cmp_leq);
        let expected_res: Result<(), EnceladusError> = Ok(());

        assert_eq!(actual_list, expected_list);
        assert_eq!(actual_res, expected_res);

        Ok(())
    }

    #[test]
    pub fn test_insertion_sort_normal_sorted_two_elems() -> Result<(), EnceladusError> {
        let mut actual_list: ArrayList<u64> = ArrayList::new();
        actual_list.append(1)?;
        actual_list.append(12)?;

        let mut expected_list: ArrayList<u64> = ArrayList::new();
        expected_list.append(1)?;
        expected_list.append(12)?;

        let actual_res: Result<(), EnceladusError> = insertion_sort(&mut actual_list,
                                                           cmp_leq);
        let expected_res: Result<(), EnceladusError> = Ok(());

        assert_eq!(actual_list, expected_list);
        assert_eq!(actual_res, expected_res);

        Ok(())
    }

}

