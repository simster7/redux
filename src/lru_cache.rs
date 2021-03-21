use std::collections::HashMap;
use crate::linked_list::{LinkedList, new_linked_list};
use std::fmt::Display;

pub struct LRUCache<'cache, T: Display> {
    map: HashMap<&'cache str, T>,
    order: LinkedList<'cache>,
    size: u32,
    max_size: u32,
}

pub fn new_lru_cache<'cache, T: Display>(max_size: u32) -> LRUCache<'cache, T> {
    return LRUCache {
        map: HashMap::new(),
        order: new_linked_list(),
        size: 0,
        max_size,
    }
}

impl<'cache, T: Display> LRUCache<'cache, T> {
    pub fn add(&mut self, key: &'cache str, value: T) {
        let already_contains = self.contains_key(key);

        self.map.insert(key, value);
        self.order.put_front(key);

        if !already_contains {
            self.size += 1;
            self._maybe_kick();
        }
    }

    pub fn remove(&mut self, key: &'cache str) {
        self.map.remove(key);
        self.size -= 1;
        self.order.remove(key);
    }

    pub fn get(&self, key: &'cache str) -> Option<&T> {
        return self.map.get(key);
    }

    pub fn contains_key(&self, key: &'cache str) -> bool {
        return self.map.contains_key(key)
    }

    pub fn print(&mut self) {
        for (key, value) in self.map.iter() {
            println!("{}: {}", key, value);
        }
        self.order.print();
        println!()
    }

    fn _maybe_kick(&mut self) {
        if self.size > self.max_size {
            if let Some(last) = self.order.pop_last() {
                self.map.remove(last.as_str());
                self.size -= 1;
            }
        }
    }
}
