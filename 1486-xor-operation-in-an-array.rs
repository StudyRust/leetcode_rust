// 示例 1：

// 输入：n = 5, start = 0
// 输出：8
// 解释：数组 nums 为 [0, 2, 4, 6, 8]，其中 (0 ^ 2 ^ 4 ^ 6 ^ 8) = 8 。
//      "^" 为按位异或 XOR 运算符。

// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/xor-operation-in-an-array
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

fn main() {
     let n = 5;
     let start = 0;
     println!("{:?}", xor_operation(n, start));
}

pub fn xor_operation(n: i32, start: i32) -> i32 {
    let mut res = start;
    for i in 1..n {
        res ^= start + 2 * i;
    }
    res
}
