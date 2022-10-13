pub fn detect_capital_use(word: String) -> bool {
    let word_len = word.len();
    let mut count = vec![0, 0];
    let chars = word.chars().collect::<Vec<char>>();
    let first_char = chars[0];
    for c in chars {
        if c.is_uppercase() {
            count[0] += 1
        } else {
            count[1] += 1
        }
    }
    if count[0] == 0 {
        true
    } else {
        if count[1] == 0 {
            true
        } else {
            count[1] == word_len - 1 && first_char.is_uppercase()
        }
    }
}

fn main() {
    println!("{:?}", detect_capital_use("USA".to_string()));
    println!("{:?}", detect_capital_use("leetcode".to_string()));
    println!("{:?}", detect_capital_use("Google".to_string()));
    println!("{:?}", detect_capital_use("FlaG".to_string()));
    println!("{:?}", detect_capital_use("ffffffffffffffffffffF".to_string()));
}
