pub fn count_words(words1: Vec<String>, words2: Vec<String>) -> i32 {
    use std::collections::HashMap;
    fn get_word_map(words: Vec<String>) -> HashMap<String, i32> {
        let mut words_map: HashMap<String, i32> = HashMap::new();
        for word in words {
            if !words_map.contains_key(&word) {
                words_map.insert(word, 1);
            } else {
                *words_map.get_mut(&word).unwrap() += 1;
            }
        }
        words_map
    }
    let mut words1_map = get_word_map(words1);
    let mut words2_map = get_word_map(words2);
    words1_map.retain(|_, v| *v == 1);
    words2_map.retain(|k, v| *v == 1 && words1_map.contains_key(k));
    words2_map.len() as i32
}

fn main() {
    let words1: Vec<String> = vec!("leetcode","is","amazing","as","is").into_iter().map(|e|e.to_string()).collect();
    let words2: Vec<String> = vec!("amazing","leetcode","is").into_iter().map(|e|e.to_string()).collect();
    println!("{:?}", count_words(words1, words2));
}
