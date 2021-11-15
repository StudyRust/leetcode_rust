// 示例 2：

// 输入：s = "111000"
// 输出：false
// 解释：
// 由 1 组成的最长连续子字符串的长度是 3："111"
// 由 0 组成的最长连续子字符串的长度是 3："000"
// 由 1 组成的子字符串不比由 0 组成的子字符串长，故返回 false 。

// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/longer-contiguous-segments-of-ones-than-zeros
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

fn main() {
    println!("{:?}", check_zero_ones("111000".to_string()));
}

pub fn check_zero_ones(s: String) -> bool {
    let (mut flag, mut tmp_0, mut tmp_1, mut max_0, mut max_1) = ('0', 0, 0, 0, 0);
    for c in s.chars() {
        if c == flag {
            if flag == '0' {
                tmp_0 += 1;
            } else {
                tmp_1 += 1;
            }
        } else {
            flag = c;
            if flag == '0' {
                tmp_0 = 1;
            } else {
                tmp_1 = 1;
            }
        }
        if tmp_0 > max_0 {
            max_0 = tmp_0;
        }
        if tmp_1 > max_1 {
            max_1 = tmp_1;
        }
    }
    max_1 > max_0
}
