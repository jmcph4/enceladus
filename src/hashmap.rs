use std::collections::HashSet;
use std::hash::Hash;

use crate::error::EnceladusError;
use crate::map::Map;

const INIT_NUM_BUCKETS: usize = 2048;

#[derive(Clone, Debug)]
struct HashMapEntry<K, V>(K, V);

type Bucket<K, V> = Vec<HashMapEntry<K, V>>;

#[derive(Clone, Debug)]
pub struct HashMap<K, V> {
    buckets: Vec<Bucket<K, V>>,
    num_keys: usize,
    load_factor: f64
}

impl<K, V> PartialEq for HashMap<K, V> where K: Sized + Eq + Clone + Hash,
    V: Sized + Eq + Clone + Hash {
    fn eq(&self, other: &Self) -> bool {
        if self.num_keys != other.num_keys ||
            self.buckets.len() != other.buckets.len() {
            return false;
        }
   
        let self_keys: HashSet<K> = self.get_keys();
        let other_keys: HashSet<K> = other.get_keys();

        let self_values: HashSet<V> = self.get_values();
        let other_values: HashSet<V> = other.get_values();

        self_keys == other_keys && self_values == other_values
    }
}

impl<K, V> Eq for HashMap<K, V> where K: Sized + Eq + Clone + Hash,
    V: Sized + Eq + Clone + Hash {}

impl<K, V> IntoIterator for HashMap<K, V> where K: Sized + Eq + Clone + Hash,
    V: Sized + Eq + Clone + Hash {
    type Item = (K, V);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.items().into_iter()
    }
    
}

impl<K, V> Map<K, V> for HashMap<K, V> where K: Sized + Eq + Clone + Hash,
    V: Sized + Eq + Clone + Hash {
    fn new() -> Self {
        let buckets_vec: Vec<Bucket<K, V>> = Vec::new();
        
        for i in 0..INIT_NUM_BUCKETS {
            buckets_vec.push(Bucket::new());
        }
        
        Self {
            buckets: buckets_vec,
            num_keys: 0,
            load_factor: 0.0
        }
    }
}

impl<K, V> HashMap<K, V> where K: Sized + Eq + Clone + Hash,
    V: Sized + Eq + Clone + Hash {
    fn update(&mut self) {
        self.load_factor = (self.num_keys / self.buckets.len()) as f64;
    }

    fn get_keys(&self) -> HashSet<K> where K: Eq + Hash {
        let mut res: HashSet<K> = HashSet::new();

        for bucket in &self.buckets {
            for entry in bucket {
                res.insert(entry.0.clone());
            }
        }

        res
    }

    fn get_values(&self) -> HashSet<V> where V: Eq + Hash {
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
            res.push((key, self.get(key).unwrap().clone()));
        }

        res
    }
}

