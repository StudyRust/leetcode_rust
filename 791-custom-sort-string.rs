pub fn custom_sort_string(order: String, s: String) -> String {
    use std::collections::HashMap;
    let mut mmp = HashMap::new();
    let order_chars = order.chars().collect::<Vec<char>>();
    for c in &order_chars {
        mmp.entry(*c).or_insert(Vec::new());
    }
    for c in s.chars().collect::<Vec<char>>() {
        mmp.entry(c).or_insert(Vec::new()).push(c.to_string());
    }
    let mut ret = String::new();
    for c in order_chars {
        let tmp = mmp.get(&c);
        if tmp != None {
            ret.push_str(&tmp.unwrap().join(""));
            mmp.remove(&c);
        }
    }
    for (_, v) in mmp {
        ret.push_str(&v.join(""));
    }
    ret
}

fn main() {
    let order = "cba".to_string();
    let s = "abcd".to_string();
    println!("{:?}", custom_sort_string(order, s));
    let order = "cbafg".to_string();
    let s = "abcd".to_string();
    println!("{:?}", custom_sort_string(order, s));
}
