pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
    for (i, w) in sentence.split(" ").enumerate() {
        if w.starts_with(&search_word) {
            return (i+1) as i32;
        }
    }
    -1
}

fn main() {
    let sentence = "this problem is an easy problem".to_string();
    let search_word = "pro".to_string();
    println!("{:?}", is_prefix_of_word(sentence, search_word));
}
