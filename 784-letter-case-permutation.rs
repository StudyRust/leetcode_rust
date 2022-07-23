// 动态规划 回溯
pub fn letter_case_permutation(s: String) -> Vec<String> {
    let mut dp: Vec<String> = vec![String::new()];
    let mut current_count = 1;
    for c in s.chars().collect::<Vec<char>>() {
        if c.is_numeric() {
            for i in dp.len()-current_count..dp.len() {
                let mut tmp = dp[i].clone();
                tmp.push(c);
                dp.push(tmp);
            }
        } else {
            for i in dp.len()-current_count..dp.len() {
                let mut tmp_lower = dp[i].clone();
                let mut tmp_upper = dp[i].clone();
                tmp_lower.push(c.to_ascii_lowercase());
                tmp_upper.push(c.to_ascii_uppercase());
                dp.push(tmp_lower);
                dp.push(tmp_upper);
            }
        }
        current_count *= 2;
    }
    dp.retain(|e|e.len() == s.len());
    dp
}

fn main() {
    let s = "a1b2".to_string();
    println!("{:?}", letter_case_permutation(s));
}
