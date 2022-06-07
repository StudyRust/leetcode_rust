pub fn max_power(s: String) -> i32 {
    let mut current_char = '!';
    let mut current_len = 1;
    let mut ret = 0;
    for c in s.chars() {
        if c == current_char {
            current_len += 1;
        } else {
            current_len = 1;
            current_char = c;
        }
        if current_len > ret {
            ret = current_len;
        }
    }
    ret
}

fn main() {
    let s = "leetcode".to_string();
    println!("{:?}", max_power(s));
}