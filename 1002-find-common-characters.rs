pub fn common_chars(words: Vec<String>) -> Vec<String> {
    let words_len = words.len();
    use std::collections::HashMap;
    let mut mmp: HashMap<String, Vec<i32>> = HashMap::new();
    for c in 'a'..'z' {
        mmp.insert(c.to_string(), vec!());
    }
    for word in words {
        let mut tmp = HashMap::new();
        let word_cs: Vec<String> = word.chars().map(|e|e.to_string()).collect();
        for c in word_cs {
            let count = tmp.entry(c).or_insert(0);
            *count += 1;
        }
        for (k, v) in tmp {
            mmp.entry(k).or_insert(Vec::new()).push(v);
        }
    }
    mmp.retain(|_, v| v.len() == words_len);
    let mut ret: Vec<String> = vec!();
    for (k, v) in mmp {
        let min = v.iter().min().unwrap();
        for _ in 0..*min {
            ret.push(k.clone());
        }
    }
    ret
}

fn main() {
    let words = vec!["bella","label","roller"]
        .iter().map(|e|e.to_string()).collect();
    println!("{:?}", common_chars(words));
}
