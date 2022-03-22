pub fn num_of_strings(patterns: Vec<String>, word: String) -> i32 {
    let mut res = 0;
    for p in patterns {
        if word.contains(&p) {
            res += 1;
        }
    }
    res
}

fn main() {
    let patterns = vec!["a","abc","bc","d"]
        .iter().map(|e|e.to_string()).collect();
    let word = "abc".to_string();
    println!("{:?}", num_of_strings(patterns, word));
}
