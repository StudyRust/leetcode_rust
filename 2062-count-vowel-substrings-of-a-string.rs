pub fn count_vowel_substrings(word: String) -> i32 {
    use std::collections::HashMap;
    let mut ret = 0;
    let vowel = vec!['a', 'e', 'i', 'o', 'u'];
    for i in 0..word.len() {
        'middle: for j in i+5..=word.len() {
            let mut mmp = HashMap::new();
            let cs = &word[i..j].to_string().clone();
            for c in cs.chars().collect::<Vec<char>>() {
                if vowel.contains(&c) {
                    let count = mmp.entry(c).or_insert(0);
                    *count += 1;
                } else {
                    continue 'middle;
                }
            }
            if mmp.len() == 5 {
                ret += 1;
            }
        }
    }
    ret
}

fn main() {
    let word = "cuaieuouac".to_string();
    println!("{:?}", count_vowel_substrings(word));
}
