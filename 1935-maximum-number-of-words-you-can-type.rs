pub fn can_be_typed_words
(text: String, broken_letters: String) -> i32
{
    fn have_broken_letters
    (word: String, broken_letters: &str) -> bool
    {
        for c in broken_letters.chars() {
            if word.contains(&c.to_string()) {
                return true;
            }
        }
        return false;
    }
    let mut res = 0;
    let words: Vec<String> = text.split(" ")
        .map(|word|word.to_string())
        .collect();

    for word in words {
        if !have_broken_letters(word, &broken_letters) {
            res += 1;
        }
    }
    res
}

fn main() {
    println!("{:?}", can_be_typed_words("hello world".to_string(), "ad".to_string()));
}
