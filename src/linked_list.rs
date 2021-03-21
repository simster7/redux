use std::collections::HashMap;

pub struct LinkedList<'list> {
    prev: HashMap<&'list str, &'list str>,
    next: HashMap<&'list str, &'list str>,
}

pub fn new_linked_list<'list>() -> LinkedList<'list> {
    let prev: HashMap<&str, &str> = HashMap::new();
    let next: HashMap<&str, &str> = HashMap::new();
    let mut linked_list = LinkedList {
        prev,
        next,
    };
    linked_list.connect("^", "$");
    return linked_list;
}

impl<'list> LinkedList<'list> {
    pub fn connect(&mut self, a: &'list str, b: &'list str) {
        self.prev.insert(b, a);
        self.next.insert(a, b);
    }

    fn _get_first(&self) -> &'list str {
        return self.next["^"];
    }

    pub fn get_first(&self) -> Option<String> {
        let first = self._get_first();
        if first == "$" {
            return None
        }
        return Some(first.into());
    }

    fn _get_last(&self) -> &'list str {
        return self.prev["$"];
    }

    pub fn get_last(&self) -> Option<String> {
        let last = self._get_last();
        if last == "^" {
            return None
        }
        return Some(last.into());
    }

    pub fn put_front(&mut self, a: &'list str) {
        if self.contains(a) {
            self.remove(a);
        }

        self.connect(a, self._get_first());
        self.connect("^", a)
    }

    pub fn pop_last(&mut self) -> Option<String> {
        if let Some(last) = self.get_last() {
            self.connect(self.prev[last.as_str()], "$");
            return Some(last);
        }
        return None
    }

    pub fn contains(&self, a: &'list str) -> bool {
        return self.prev.contains_key(a)
    }

    pub fn remove(&mut self, a: &'list str) {
        self.connect(self.prev[a], self.next[a]);
    }

    pub fn print(&self) {
        let mut curr = self.next["^"];
        while curr != "$" {
            print!("{} ", curr);
            curr = self.next[curr];
        }
        println!()
    }
}
