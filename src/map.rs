use crate::error::EnceladusError;

pub trait Map<K: Sized + Eq + Clone, V: Sized + Eq + Clone>: IntoIterator +
    Eq + Clone {
    fn new() -> Self;
    fn get(&self, key: K) -> Result<Option<&V>, EnceladusError>;
    fn get_mut(&mut self, key: K) -> Result<Option<&mut V>, EnceladusError>;
    fn set(&mut self, key: K, value: V) -> Result<(), EnceladusError>;
    fn insert(&mut self, key: K, value: V) -> Result<(), EnceladusError>;
    fn remove(&mut self, key: K) -> Result<(), EnceladusError>;
    fn size(&self) -> Result<usize, EnceladusError>;
    fn contains_key(&self, key: K) -> Result<bool, EnceladusError>;
    fn contains_value(&self, value: V) -> Result<bool, EnceladusError>;
    fn clear(&mut self) -> Result<(), EnceladusError>;
}

