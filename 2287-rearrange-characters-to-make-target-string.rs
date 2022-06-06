pub fn rearrange_characters(s: String, target: String) -> i32 {
    use std::collections::HashMap;
    let mut s_mmp = HashMap::new();
    let mut target_mmp = HashMap::new();
    for c in s.chars() {
        let count = s_mmp.entry(c).or_insert(0);
        *count += 1;
    }
    for c in target.chars() {
        let count = target_mmp.entry(c).or_insert(0);
        *count += 1;
    }
    let mut arr = vec!();
    for (k, v) in target_mmp {
        let tmp = s_mmp.get(&k);
        if tmp.is_some() {
            arr.push(tmp.unwrap()/v);
        } else {
            arr.push(0);
        }
    }
    *arr.iter().min().unwrap()
}

fn main() {
    let s = "abcd".to_string();
    let target = "abcd".to_string();
    println!("{:?}", rearrange_characters(s, target));
}
