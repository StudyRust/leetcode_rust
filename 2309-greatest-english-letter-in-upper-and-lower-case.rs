pub fn greatest_letter(s: String) -> String {
    use std::collections::HashMap;
    let mut s_chars = s.chars().collect::<Vec<char>>();
    s_chars.sort_unstable();
    s_chars.dedup();
    let mut mmp = HashMap::new();
    let mut ret = '0';
    for c in s_chars {
        let cu = c.to_ascii_uppercase();
        let count = mmp.entry(cu).or_insert(0);
        *count += 1;
        if *count > 1 && cu > ret { ret = cu }
    }
    if ret == '0' { return String::new() }
    ret.to_string()
}

fn main() {
    println!("{:?}", greatest_letter("AbCdEfGhIjK".to_string()));
}
