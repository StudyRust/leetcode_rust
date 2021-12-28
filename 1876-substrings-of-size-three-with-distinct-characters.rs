pub fn count_good_substrings(s: String) -> i32 {
    let mut res = 0;
    if s.len() >= 3 {
        for i in 0..s.len()-2 {
            let tmp_str = &s[i..i+3];
            let s_len = tmp_str.len();
            let mut tmp_chars: Vec<char> = tmp_str.chars().collect();
            tmp_chars.sort_unstable();
            tmp_chars.dedup();
            if tmp_chars.len() == s_len {
                res += 1;
            }
        }
    }
    res
}

fn main() {
    println!("{}", count_good_substrings("x".to_string()));
}
