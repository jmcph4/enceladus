#[derive(Clone, Debug)]
struct HashMapEntry<K, V>(K, V);

#[derive(Clone, Debug)]
pub struct HashMap<K, V> {
    buckets: Vec<Vec<HashMapEntry<K, V>>>,
    num_keys: usize,
    num_values: usize,
    load_factor: f64
}

