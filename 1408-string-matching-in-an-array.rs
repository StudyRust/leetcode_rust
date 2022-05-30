pub fn string_matching(words: Vec<String>) -> Vec<String> {
    let mut ret: Vec<String> = vec!();
    for word in &words {
        for w in &words {
            if word != w && w.contains(word) {
                ret.push(word.to_string());
                break;
            }
        }
    }
    ret
}

fn main() {
    let words: Vec<String> = vec!["leetcoder","leetcode","od","hamlet","am"]
        .iter().map(|e|e.to_string()).collect();
    println!("{:?}", string_matching(words));
}
