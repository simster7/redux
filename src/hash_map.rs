use std::fmt::Display;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

pub struct HashMap<'map, V: Display+Clone> {
    bins: Vec<Vec<Node<'map, V>>>,
    size: u32,
}

pub fn new_hash_map<'map, V: Display+Clone>() -> HashMap<'map, V> {
    let inner = Vec::new();
    let outer = vec![inner; 10];
    return HashMap {
        bins: outer,
        size: 0,
    }
}

fn hash(key: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher);
    return hasher.finish();
}

impl<'map> HashMap<'map, V> {
    pub fn insert(&mut self, key: &'map, value: V) {
        let hash = hash(key);
        let bin_index = hash % self.bins.len();
        let bin = &self.bins[bin_index];
        

    }
}


#[derive(Clone)]
struct Node<'node, V: Display+Clone> {
    key: &'node str,
    value: V,
}

fn new_node<'node, V: Display+Clone>(key: K, value: V) -> Node<'node, V> {
    return Node {
        key,
        value,
    }
}
