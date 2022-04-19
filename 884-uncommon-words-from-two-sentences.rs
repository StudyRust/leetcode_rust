pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
    use std::collections::HashMap;
    let mut mmp = HashMap::new();
    for w in s1.split_whitespace() {
        let count = mmp.entry(w).or_insert(0);
        *count += 1;
    }
    for w in s2.split_whitespace() {
        let count = mmp.entry(w).or_insert(0);
        *count += 1;
    }
    mmp.retain(|_, v| *v == 1);
    mmp.into_keys()
        .collect::<Vec<&str>>()
        .iter()
        .map(|e|e.to_string())
        .collect()
}

fn main() {
    let s1 = "this apple is sweet".to_string();
    let s2 = "this apple is sour".to_string();
    println!("{:?}", uncommon_from_sentences(s1, s2));
}
