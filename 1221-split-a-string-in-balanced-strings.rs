// 示例 1：

// 输入：s = "RLRRLLRLRL"
// 输出：4
// 解释：s 可以分割为 "RL"、"RRLL"、"RL"、"RL" ，每个子字符串中都包含相同数量的 'L' 和 'R' 。

// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/split-a-string-in-balanced-strings
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
// RLLLLRRRLR   3
// LLLLRRRR     1
// RLRRRLLRLL   2


fn main() {
    let str = "RLRRRLLRLL".to_string();
    let res = balanced_string_split(str);
    println!("{:?}", res);
}

pub fn balanced_string_split(s: String) -> i32 {
    let mut r_count = 0;
    let mut l_count = 0;
    let mut res = 0;
    for char in s.chars() {
        if char == 'R' {
            r_count += 1;
        } else {
            l_count += 1;
        }
        if l_count == r_count {
            res += 1;
            r_count = 0;
            l_count = 0;
        }
    }
    res
}