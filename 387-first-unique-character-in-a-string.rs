pub fn first_uniq_char(s: String) -> i32 {
    use std::collections::HashMap;
    let mut mmp = HashMap::new();
    for (i, c) in s.chars().enumerate() {
        mmp.entry(c).or_insert(Vec::new()).push(i);
    }
    mmp.retain(|_, v|v.len() == 1);
    if mmp.len() > 0 {
        mmp.into_values().map(|e|e[0]).min().unwrap() as i32
    } else {
        -1
    }
}

fn main() {
    println!("{:?}", first_uniq_char("ccbb".to_string()));
}
