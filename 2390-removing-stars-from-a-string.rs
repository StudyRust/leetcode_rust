pub fn remove_stars(s: String) -> String {
    let mut ret = String::new();
    for c in s.chars().collect::<Vec<char>>() {
        if c == '*' { ret.pop(); } else { ret.push(c); }
    }
    ret
}

fn main() {
    println!("{:?}", remove_stars("leet**cod*e".to_string()));
}
