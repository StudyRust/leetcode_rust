pub fn max_length_between_equal_characters(s: String) -> i32 {
    use std::collections::HashMap;
    let mut mmp = HashMap::new();
    let cs: Vec<char> = s.chars().collect();
    for (i, c) in cs.iter().enumerate() {
        mmp.entry(c).or_insert(Vec::new()).push(i);
    }
    mmp.retain(|_, v|v.len()>1);
    if mmp.len() > 0 {
        let vecs: Vec<_> = mmp.into_values().collect();
        let v: Vec<_> = vecs.iter().map(|e|e.last().unwrap() - e.first().unwrap()).collect();
        *v.iter().max().unwrap() as i32 - 1
    } else {
        return -1
    }
}

fn main() {
    let s = "abcd".to_string();
    println!("{:?}", max_length_between_equal_characters(s));
}