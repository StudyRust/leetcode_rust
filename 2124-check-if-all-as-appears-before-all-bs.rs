pub fn check_string(s: String) -> bool {
    let s_chars: Vec<char> = s.chars().collect();
    let mut sorted: Vec<char> = s_chars.clone();
    sorted.sort();
    sorted == s_chars
}

fn main() {
    let s = "aaaa".to_string();
    println!("{:?}", check_string(s));
}
