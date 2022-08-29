use std::collections::HashMap;

struct MyHashMap {
    map: HashMap<i32, i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashMap {

    /** Initialize your data structure here. */
    fn new() -> Self {
        MyHashMap{map: HashMap::new()}
    }

    /** value will always be non-negative. */
    fn put(&mut self, key: i32, value: i32) {
        self.map.insert(key, value);
    }

    /** Returns the value to which the specified key is mapped, or -1 if this map contains no mapping for the key */
    fn get(&self, key: i32) -> i32 {
        if self.map.get(&key).is_some() {
            *self.map.get(&key).unwrap()
        } else {
            -1
        }
    }

    /** Removes the mapping of the specified value key if this map contains a mapping for the key */
    fn remove(&mut self, key: i32) {
        self.map.remove(&key);
    }
}

fn main() {
    let mut obj = MyHashMap::new();
    obj.put(1, 2);
    let ret_2: i32 = obj.get(1);
    obj.remove(1);
}
