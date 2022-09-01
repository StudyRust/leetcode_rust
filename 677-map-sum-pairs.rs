use std::collections::HashMap;

struct MapSum {
    mmp: HashMap<String, i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MapSum {

    fn new() -> Self {
        MapSum {
            mmp: HashMap::new()
        }
    }

    fn insert(&mut self, key: String, val: i32) {
        if let Some(x) = self.mmp.get_mut(&key) {
            *x = val;
        } else {
            self.mmp.insert(key, val);
        }
    }

    fn sum(&mut self, prefix: String) -> i32 {
        let mut ret = 0;
        for (k, v) in &self.mmp {
            if k.starts_with(&prefix) {
                ret += v;
            }
        }
        ret
    }
}

fn main() {
    let mut obj = MapSum::new();
    obj.insert("apple".to_string(), 2);
    println!("{:?}", obj.sum("ap".to_string()));
    obj.insert("app".to_string(), 3);
    obj.insert("apple".to_string(), 3);
    println!("{:?}", obj.sum("ap".to_string()));
}
