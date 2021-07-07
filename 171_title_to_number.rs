fn main() {
    let str = "FXSHRXW".to_string();
    let a = title_to_number(str);
    println!("{:?}", a);
}

// 171 https://leetcode-cn.com/problems/excel-sheet-column-number/submissions/
// "FXSHRXW" 2147483647
pub fn title_to_number(column_title: String) -> i32 {
    let mut sum = 0;
    for (i, c) in column_title.chars().rev().enumerate() {
        sum += 26_i32.pow(i as u32) * (c as i32 - 64)
    };
    sum
}