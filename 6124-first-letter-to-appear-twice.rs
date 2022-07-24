pub fn repeated_character(s: String) -> char {
    use std::collections::HashMap;
    let mut mp = HashMap::new();
    let mut ret = '!';
    for c in s.chars().collect::<Vec<char>>() {
        let count = mp.entry(c).or_insert(0);
        *count += 1;
        if *count == 2 {
            ret = c;
            break;
        }
    }
    ret
}

fn main() {
    println!("{:?}", repeated_character("abccbaacz".to_string()));
}
