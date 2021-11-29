// 示例 1：

// 输入：
// 上边界left = 1, 下边界right = 22
// 输出： [1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 15, 22]

// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/self-dividing-numbers
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
    let mut res = vec!();
    for number in left..right+1 {
        let digit_chars: Vec<char> = number.to_string().chars().collect();
        if digit_chars.contains(&'0') {
            continue;
        }
        let mut flag = true;
        for c in digit_chars {
            if number % c.to_digit(10).unwrap() as i32 != 0 {
                flag = false;
                break;
            }
        }
        if flag {
            res.push(number);
        }
    }
    res
}

fn main() {
    println!("{:?}", self_dividing_numbers(1, 22));
}
