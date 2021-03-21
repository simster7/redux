use crate::lru_cache::new_lru_cache;
use std::io::stdin;

mod lru_cache;
mod linked_list;

fn main() {
    // println!("Cache size?");
    // let size: u32 = read_line().parse().unwrap();
    let mut cache = new_lru_cache(5);
    cache.add("1", 1);
    cache.add("2", 2);
    cache.add("3", 3);
    cache.add("4", 4);
    cache.add("5", 5);
    cache.print();
    println!("Full here");
    cache.add("6", 6);
    cache.add("7", 7);
    cache.print();

}

fn read_line() -> String {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer);
    return buffer.trim_end().into();
}
