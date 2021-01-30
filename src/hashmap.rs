//! An array-based separate chaining hash table, implementing the map ADT. Uses
//! Rust's `Vec` internally

use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

use crate::error::EnceladusError;
use crate::map::Map;

const INIT_NUM_BUCKETS: usize = 64;
const GROWTH_FACTOR: usize = 2;
const LOAD_THRESHOLD: f64 = 0.75;

#[derive(Clone, Debug)]
struct HashMapEntry<K, V>(K, V);

type Bucket<K, V> = Vec<HashMapEntry<K, V>>;

#[derive(Clone, Debug)]
pub struct HashMap<K, V> {
    buckets: Vec<Bucket<K, V>>,
    num_keys: usize,
    load_factor: f64,
}

impl<K, V> PartialEq for HashMap<K, V>
where
    K: Sized + Eq + Clone + Hash,
    V: Sized + Eq + Clone + Hash,
{
    fn eq(&self, other: &Self) -> bool {
        if self.num_keys != other.num_keys
            || self.buckets.len() != other.buckets.len()
        {
            return false;
        }

        let self_keys: HashSet<K> = self.get_keys();
        let other_keys: HashSet<K> = other.get_keys();

        let self_values: HashSet<V> = self.get_values();
        let other_values: HashSet<V> = other.get_values();

        self_keys == other_keys && self_values == other_values
    }
}

impl<K, V> Eq for HashMap<K, V>
where
    K: Sized + Eq + Clone + Hash,
    V: Sized + Eq + Clone + Hash,
{
}

impl<K, V> IntoIterator for HashMap<K, V>
where
    K: Sized + Eq + Clone + Hash,
    V: Sized + Eq + Clone + Hash,
{
    type Item = (K, V);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.items().into_iter()
    }
}

impl<K, V> Map<K, V> for HashMap<K, V>
where
    K: Sized + Eq + Clone + Hash,
    V: Sized + Eq + Clone + Hash,
{
    fn new() -> Self {
        let mut buckets_vec: Vec<Bucket<K, V>> = Vec::new();

        for _i in 0..INIT_NUM_BUCKETS {
            buckets_vec.push(Bucket::new());
        }

        Self {
            buckets: buckets_vec,
            num_keys: 0,
            load_factor: 0.0,
        }
    }

    /// Retrieves the value associated with the provided key and `None` if the
    /// key is not found.
    ///
    /// # Performance #
    ///
    /// |       | Best | Average | Worst |
    /// |-------|------|---------|-------|
    /// | Time  | O(1) | O(1)    | O(n)  |
    /// | Space | O(1) | O(1)    | O(1)  |
    ///
    /// The linear worst-case time complexity for this method is due to the
    /// possibility (although **extremely** unlikely), that retrieval will
    /// require a full traversal of a bucket that contains every key-value pair.
    fn get(&self, key: K) -> Result<Option<&V>, EnceladusError> {
        /* hash key to derive bucket index */
        let mut hasher: DefaultHasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let bucket_index: usize = hasher.finish() as usize % self.buckets.len();

        if bucket_index >= self.buckets.len() {
            /* bounds check */
            return Err(EnceladusError::OutOfBounds);
        }

        let target_bucket: &Bucket<K, V> = &self.buckets[bucket_index];

        /* linear scan over the bucket, searching for matching entry */
        for entry in target_bucket {
            if entry.0 == key {
                return Ok(Some(&entry.1));
            }
        }

        Ok(None)
    }

    /// Provides a mutable view to the value associated with the provided key
    /// and `None` if the key is not found.
    ///
    /// # Performance #
    ///
    /// |       | Best | Average | Worst |
    /// |-------|------|---------|-------|
    /// | Time  | O(1) | O(1)    | O(n)  |
    /// | Space | O(1) | O(1)    | O(1)  |
    ///
    /// The linear worst-case time complexity for this method is due to the
    /// possibility (although **extremely** unlikely), that retrieval will
    /// require a full traversal of a bucket that contains every key-value pair.
    fn get_mut(&mut self, key: K) -> Result<Option<&mut V>, EnceladusError> {
        /* hash key to derive bucket index */
        let mut hasher: DefaultHasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let bucket_index: usize = hasher.finish() as usize % self.buckets.len();

        if bucket_index >= self.buckets.len() {
            /* bounds check */
            return Err(EnceladusError::OutOfBounds);
        }

        let target_bucket: &mut Bucket<K, V> = &mut self.buckets[bucket_index];

        /* linear scan over the bucket, searching for matching entry */
        for entry in target_bucket.iter_mut() {
            if entry.0 == key {
                return Ok(Some(&mut entry.1));
            }
        }

        Ok(None)
    }

    /// Sets the value associated with the provided key.
    ///
    /// # Performance #
    ///
    /// |       | Best | Average | Worst |
    /// |-------|------|---------|-------|
    /// | Time  | O(1) | O(1)    | O(n)  |
    /// | Space | O(1) | O(1)    | O(1)  |
    ///
    /// The linear worst-case time complexity for this method is due to the
    /// possibility (although **extremely** unlikely), that updating will
    /// require a full traversal of a bucket that contains every key-value pair.
    fn set(&mut self, key: K, value: V) -> Result<(), EnceladusError> {
        /* hash key to derive bucket index */
        let mut hasher: DefaultHasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let bucket_index: usize = hasher.finish() as usize % self.buckets.len();

        if bucket_index >= self.buckets.len() {
            /* bounds check */
            return Err(EnceladusError::OutOfBounds);
        }

        let target_bucket: &mut Bucket<K, V> = &mut self.buckets[bucket_index];

        /* linear scan over the bucket, searching for matching entry */
        for entry in target_bucket {
            if entry.0 == key {
                entry.1 = value;
                return Ok(());
            }
        }

        Err(EnceladusError::KeyNotFound)
    }

    /// Inserts the key-value mapping into the hash table
    ///
    /// # Performance #
    ///
    /// |       | Best | Average | Worst |
    /// |-------|------|---------|-------|
    /// | Time  | O(1) | O(1)    | O(n)  |
    /// | Space | O(1) | O(1)    | O(1)  |
    ///
    /// Note that this method uses constant space **per call**. Obviously the
    /// overall space complexity of adding additional key-value entries into the
    /// hash table is O(n) in the number of entries.
    ///
    /// The linear worst-case time complexity for this method is due to the
    /// possibility (although **extremely** unlikely), that insertion will
    /// require a full traversal of a bucket that contains every key-value pair.
    fn insert(&mut self, key: K, value: V) -> Result<(), EnceladusError> {
        /* hash key to derive bucket index */
        let mut hasher: DefaultHasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let bucket_index: usize = hasher.finish() as usize % self.buckets.len();

        if bucket_index >= self.buckets.len() {
            /* bounds check */
            return Err(EnceladusError::OutOfBounds);
        }

        let target_bucket: &mut Bucket<K, V> = &mut self.buckets[bucket_index];

        /* build new entry and append onto target bucket */
        let new_entry: HashMapEntry<K, V> = HashMapEntry(key, value);
        target_bucket.push(new_entry);
        self.num_keys += 1;
        self.update();

        Ok(())
    }

    /// Removes the key-value mapping from the hash table.
    ///
    /// # Errors #
    ///
    /// - `EnceladusError::KeyNotFound` if `key` is not currently stored within
    ///     the hash table
    ///
    /// # Performance #
    ///
    /// |       | Best | Average | Worst |
    /// |-------|------|---------|-------|
    /// | Time  | O(1) | O(1)    | O(n)  |
    /// | Space | O(1) | O(1)    | O(1)  |
    ///
    /// The linear worst-case time complexity for this method is due to the
    /// possibility (although **extremely** unlikely), that removal will
    /// require a full traversal of a bucket that contains every key-value pair.
    fn remove(&mut self, key: K) -> Result<(), EnceladusError> {
        /* hash key to derive bucket index */
        let mut hasher: DefaultHasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let bucket_index: usize = hasher.finish() as usize % self.buckets.len();

        if bucket_index >= self.buckets.len() {
            /* bounds check */
            return Err(EnceladusError::OutOfBounds);
        }

        let target_bucket: &mut Bucket<K, V> = &mut self.buckets[bucket_index];

        /* linear scan over the bucket, searching for matching entry */
        for (i, entry) in target_bucket.iter().enumerate() {
            if entry.0 == key {
                target_bucket.remove(i);
                self.num_keys -= 1;
                self.update();
                return Ok(());
            }
        }

        Err(EnceladusError::KeyNotFound)
    }

    /// Returns the number of key-value pairs stored within the hash table.
    ///
    /// # Performance #
    ///
    /// |       | Best | Average | Worst |
    /// |-------|------|---------|-------|
    /// | Time  | O(1) | O(1)    | O(1)  |
    /// | Space | O(1) | O(1)    | O(1)  |
    fn size(&self) -> Result<usize, EnceladusError> {
        Ok(self.num_keys)
    }

    /// Determines whether or not the provided key is currently stored within
    /// the hash table.
    ///
    /// # Performance #
    ///
    /// |       | Best | Average | Worst |
    /// |-------|------|---------|-------|
    /// | Time  | O(n) | O(n)    | O(n)  |
    /// | Space | O(n) | O(n)    | O(n)  |
    fn contains_key(&self, key: K) -> Result<bool, EnceladusError> {
        let keys: HashSet<K> = self.get_keys();

        Ok(keys.contains(&key))
    }

    /// Determines whether or not the provided value is currently stored within
    /// the hash table.
    ///
    /// # Performance #
    ///
    /// |       | Best | Average | Worst |
    /// |-------|------|---------|-------|
    /// | Time  | O(n) | O(n)    | O(n)  |
    /// | Space | O(n) | O(n)    | O(n)  |
    fn contains_value(&self, value: V) -> Result<bool, EnceladusError> {
        let values: HashSet<V> = self.get_values();

        Ok(values.contains(&value))
    }

    /// Removes all key-value pairs from the hash table.
    ///
    /// # Performance #
    ///
    /// |       | Best | Average | Worst |
    /// |-------|------|---------|-------|
    /// | Time  | O(1) | O(1)    | O(1)  |
    /// | Space | O(1) | O(1)    | O(1)  |
    ///
    /// Uses Rust's `Vec::clear` internally to clear bucket container.
    fn clear(&mut self) -> Result<(), EnceladusError> {
        self.buckets.clear();
        self.num_keys = 0;
        self.update();
        Ok(())
    }
}

impl<K, V> HashMap<K, V>
where
    K: Sized + Eq + Clone + Hash,
    V: Sized + Eq + Clone + Hash,
{
    /// Returns the current load factor of the hash table
    ///
    /// # Performance #
    /// |       | Best | Average | Worst |
    /// |-------|------|---------|-------|
    /// | Time  | O(1) | O(1)    | O(1)  |
    /// | Space | O(1) | O(1)    | O(1)  |
    ///
    /// This method is constant-time as load factor is stored as table metadata.
    pub fn load_factor(&self) -> f64 {
        self.load_factor
    }

    fn update(&mut self) {
        let mut occupied_buckets: u64 = 0;

        /* count number of nonempty buckets */
        for bucket in self.buckets.iter() {
            if bucket.len() > 0 {
                occupied_buckets += 1;
            }
        }

        self.load_factor = occupied_buckets as f64 / self.buckets.len() as f64;

        if self.load_factor >= LOAD_THRESHOLD {
            self.rehash();
        }
    }

    fn rehash(&mut self) {
        let new_num_buckets: usize = self.buckets.len() * GROWTH_FACTOR;
        let items: Vec<(K, V)> = self.items();

        self.buckets.clear();
        self.buckets = Vec::new();

        /* initalise new buckets */
        for _i in 0..new_num_buckets {
            self.buckets.push(Bucket::new());
        }

        /* restore data */
        for (key, value) in items {
            self.insert(key, value).unwrap();
        }
    }

    fn get_keys(&self) -> HashSet<K>
    where
        K: Eq + Hash,
    {
        let mut res: HashSet<K> = HashSet::new();

        for bucket in &self.buckets {
            for entry in bucket {
                res.insert(entry.0.clone());
            }
        }

        res
    }

    fn get_values(&self) -> HashSet<V>
    where
        V: Eq + Hash,
    {
        let mut res: HashSet<V> = HashSet::new();

        for bucket in &self.buckets {
            for entry in bucket {
                res.insert(entry.1.clone());
            }
        }

        res
    }

    fn items(&self) -> Vec<(K, V)> {
        let mut res: Vec<(K, V)> = Vec::new();

        for key in self.get_keys() {
            res.push((key.clone(), self.get(key).unwrap().unwrap().clone()));
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_normal() -> Result<(), EnceladusError> {
        let mut actual_hashmap: HashMap<String, usize> = HashMap::new();

        let key: String = "Sally".to_string();
        let value: usize = key.len() as usize;

        let actual_res: Result<(), EnceladusError> =
            actual_hashmap.insert(key, value);

        let expected_res: Result<(), EnceladusError> = Ok(());

        assert_eq!(actual_res, expected_res);
        assert_eq!(actual_hashmap.size()?, 1);
        Ok(())
    }

    #[test]
    fn test_remove_normal() -> Result<(), EnceladusError> {
        let mut actual_hashmap: HashMap<String, usize> = HashMap::new();

        let key: String = "Sally".to_string();
        let value: usize = key.len() as usize;

        actual_hashmap.insert(key.clone(), value)?;

        let actual_res: Result<(), EnceladusError> = actual_hashmap.remove(key);

        let expected_res: Result<(), EnceladusError> = Ok(());
        let expected_hashmap: HashMap<String, usize> = HashMap::new();

        assert_eq!(actual_res, expected_res);
        assert_eq!(actual_hashmap, expected_hashmap);
        Ok(())
    }

    #[test]
    fn test_insert_duplicate() -> Result<(), EnceladusError> {
        let mut actual_hashmap: HashMap<String, usize> = HashMap::new();

        let key: String = "Sally".to_string();
        let value: usize = key.len() as usize;

        actual_hashmap.insert(key.clone(), value)?;

        let actual_res: Result<(), EnceladusError> =
            actual_hashmap.insert(key.clone(), value + 1);

        let expected_res: Result<(), EnceladusError> = Ok(());

        let mut expected_hashmap: HashMap<String, usize> = HashMap::new();
        expected_hashmap.insert(key.clone(), value + 1)?;

        assert_eq!(actual_res, expected_res);
        Ok(())
    }

    #[test]
    fn test_remove_nonexistant() -> Result<(), EnceladusError> {
        let mut actual_hashmap: HashMap<String, usize> = HashMap::new();

        let key: String = "Sally".to_string();
        let _value: usize = key.len() as usize;

        let actual_res: Result<(), EnceladusError> = actual_hashmap.remove(key);

        let expected_res: Result<(), EnceladusError> =
            Err(EnceladusError::KeyNotFound);
        let expected_hashmap: HashMap<String, usize> = HashMap::new();

        assert_eq!(actual_res, expected_res);
        assert_eq!(actual_hashmap, expected_hashmap);
        Ok(())
    }
}
