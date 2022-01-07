pub fn find_the_difference(s: String, t: String) -> char {
    use std::collections::HashMap;
    let mut chars_map = HashMap::new();
    for c in s.chars() {
        let counter = chars_map.entry(c).or_insert(0);
        *counter += 1;
    }
    for c in t.chars() {
        let counter = chars_map.entry(c).or_insert(0);
        *counter -= 1;
    }
    chars_map.retain(|_, v| v == &-1);
    *chars_map.keys().nth(0).unwrap()
}

fn main() {
    let s = "abcd".to_string();
    let t = "abcde".to_string();
    println!("{:?}", find_the_difference(s, t));
}
