pub fn is_circular_sentence(sentence: String) -> bool {
    if sentence.chars().nth(0).unwrap() != 
        sentence.chars().nth(sentence.len() - 1).unwrap() {
        return false
    }
    let mut count = 0;
    let mut prev_char: char = '?';
    for word in sentence.split_ascii_whitespace() {
        if count == 0 {
            prev_char = word.chars().last().unwrap()
        } else {
            let curr_char = word.chars().nth(0).unwrap();
            if curr_char != prev_char { return false }
            prev_char = word.chars().last().unwrap();
        }
        count += 1
    }
    true
}

fn main() {
    let sentence = "leetcode exercises sound delightful".to_string();
    println!("{:?}", is_circular_sentence(sentence));
    let sentence = "eetcode".to_string();
    println!("{:?}", is_circular_sentence(sentence));
}
