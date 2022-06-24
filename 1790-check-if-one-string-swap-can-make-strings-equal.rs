pub fn are_almost_equal(s1: String, s2: String) -> bool {
    if s1 == s2 {
        return true;
    }
    let mut diff_idx = vec!();
    let s1_chars = s1.chars().collect::<Vec<char>>();
    let s2_chars = s2.chars().collect::<Vec<char>>();
    for i in 0..s1.len() {
        if s1_chars[i] != s2_chars[i] {
            diff_idx.push(i);
        }
        if diff_idx.len() > 2 {
            return false;
        }
    }
    diff_idx.len() == 2 &&
    s1_chars[diff_idx[0]] == s2_chars[diff_idx[1]] &&
    s1_chars[diff_idx[1]] == s2_chars[diff_idx[0]]
}

fn main() {
    let s1 = "aa".to_string();
    let s2 = "ac".to_string();
    println!("{:?}", are_almost_equal(s1, s2));
}
