pub fn prefix_count(words: Vec<String>, pref: String) -> i32 {
    let mut counter = 0;
    for word in words {
        if word.starts_with(&pref) {
            counter += 1;
        }
    }
    counter
}

fn main() {
    let words = vec!["pay","attention","practice","attend"].iter().map(|e|e.to_string()).collect();
    let pref = "at".to_string();
    println!("{:?}", prefix_count(words, pref));
}
