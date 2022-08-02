pub fn make_equal(words: Vec<String>) -> bool {
    let words_len = words.len();
    if words_len == 1 { return true }
    use std::collections::HashMap;
    let mut mmp = HashMap::new();
    for word in words {
        for c in word.chars().collect::<Vec<char>>() {
            let count = mmp.entry(c).or_insert(0);
            *count += 1;
        }
    }
    mmp.retain(|_, v| *v % words_len != 0);
    mmp.is_empty()
}

fn main() {
    let words = ["abc","aabc","bc"].iter().map(|e|e.to_string()).collect::<Vec<String>>();
    println!("{:?}", make_equal(words));
}
