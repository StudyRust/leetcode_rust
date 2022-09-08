// 数学归纳法：每次遇到不同数位，结果+1
// 如果target从0开始，结果初始为0，从1，结果初始为1

pub fn min_flips(target: String) -> i32 {
    let mut prev = target.chars().nth(0).unwrap();
    let mut ret = prev as i32 - '0' as i32;
    for c in target.chars().collect::<Vec<char>>() {
        if c != prev { ret += 1 }
        prev = c;
    }
    ret
}

fn main() {
    let target = "10111".to_string();
    println!("{:?}", min_flips(target));
}
