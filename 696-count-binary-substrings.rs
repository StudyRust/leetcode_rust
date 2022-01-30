// 先统计连续的0和1分别有多少个
// 如：111100011000，得到4323
// 在4323中的任意相邻两个数字，
// 取小的一个加起来，就是3+2+2 = 7

pub fn count_binary_substrings(s: String) -> i32 {
    let mut cur_char = 'w';
    let mut tmp_count = 0;
    let mut arr = vec!();
    for c in s.chars() {
        if c == cur_char {
            tmp_count += 1;
        } else {
            if tmp_count != 0 {
                arr.push(tmp_count);    
            }
            cur_char = c;
            tmp_count = 1;
        }
    }
    arr.push(tmp_count);
    let mut res = 0;
    for i in 0..(&arr.len()-1) {
        if arr[i] < arr[i+1] {
            res += arr[i];
        } else {
            res += arr[i+1];
        }
    }
    res
}

fn main() {
    let s = "00110011".to_string();
    println!("{:?}", count_binary_substrings(s));
}