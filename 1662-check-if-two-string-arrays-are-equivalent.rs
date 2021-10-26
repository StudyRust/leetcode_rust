fn main() {
    let word1 = vec!("ab".to_string(), "c".to_string());
    let word2 = vec!("a".to_string(), "bc".to_string());
    println!("{:?}", array_strings_are_equal(word1, word2));
}

pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
    word1.join("") == word2.join("")
}
