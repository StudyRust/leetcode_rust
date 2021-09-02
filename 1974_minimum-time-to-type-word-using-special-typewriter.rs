fn main() {
    let str = "zjpc".to_string();
    let a = min_time_to_type(str);
    println!("{:?}", a);
}

// 1974 https://leetcode-cn.com/problems/minimum-time-to-type-word-using-special-typewriter/
// word = "abc"     5
// word = "bza"     7
// word = "zjpc"    34
pub fn min_time_to_type(word: String) -> i32 {
    let mut sum = word.len() as i32;
    let mut pre_c = 'a';
    for c in word.chars() {
        if c as i32 >= pre_c as i32 {
            if c as i32 - pre_c as i32 <= 26 - c as i32 + pre_c as i32 {
                sum += c as i32 - pre_c as i32;
            } else {
                sum += 26 - c as i32 + pre_c as i32;
            }
        } else {
            if pre_c as i32 - c as i32 <= 26 - pre_c as i32 + c as i32 {
                sum += pre_c as i32 - c as i32;
            } else {
                sum += 26 - pre_c as i32 + c as i32;
            }
        }
        pre_c = c;
    };
    sum
}