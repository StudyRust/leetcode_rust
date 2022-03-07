pub fn first_palindrome(words: Vec<String>) -> String {
    fn is_palindrome(word: &String) -> bool {
        let word_chars: Vec<char> = word.clone().chars().collect();
        let word_chars_rev: Vec<char> = word.chars().rev().collect();
        word_chars == word_chars_rev
    }
    for word in words {
        if is_palindrome(&word) {
            return word;
        }
    }
    "".to_string()
}

fn main() {
    let words = vec!["notapalindrome".to_string(), "racescar".to_string()];
    println!("{:?}", first_palindrome(words));
}
