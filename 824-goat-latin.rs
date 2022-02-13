pub fn to_goat_latin(sentence: String) -> String {
    let mut res: Vec<String> = vec!();
    let words: Vec<String> = sentence.split(" ").map(|e|e.to_string()).collect();
    for (i, word) in words.iter().enumerate() {
        let mut tmp = word.clone();
        if word.starts_with("a") || word.starts_with("e") ||
            word.starts_with("i") || word.starts_with("o") ||
            word.starts_with("u") || word.starts_with("A") ||
            word.starts_with("E") || word.starts_with("I") ||
            word.starts_with("O") || word.starts_with("U") {
        } else {
            let mut word_chars: Vec<char> = word.chars().collect();
            word_chars.rotate_left(1);
            tmp = word_chars.iter().collect();
        }
        tmp.push_str("ma");
        tmp.push_str(&"a".repeat(i+1));
        res.push(tmp);
    }
    res.join(" ")
}

fn main() {
    let sentence = "The quick brown fox jumped over the lazy dog".to_string();
    println!("{:?}", to_goat_latin(sentence));
}
