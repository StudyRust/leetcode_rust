// 给定一个正整数，检查它的二进制表示是否总是 0、1 交替出现：换句话说，就是二进制表示中相邻两位的数字永不相同。

// 示例 1：

// 输入：n = 5
// 输出：true
// 解释：5 的二进制表示是：101

// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/binary-number-with-alternating-bits
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

pub fn has_alternating_bits(n: i32) -> bool {
    let bin_str = format!("{:b}", n);
    !bin_str.contains("00") && !bin_str.contains("11")
}

fn main() {
    println!("{:?}", has_alternating_bits(7));
}
