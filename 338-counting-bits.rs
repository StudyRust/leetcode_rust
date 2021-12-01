// 示例 2：

// 输入：n = 5
// 输出：[0,1,1,2,1,2]
// 解释：
// 0 --> 0
// 1 --> 1
// 2 --> 10
// 3 --> 11
// 4 --> 100
// 5 --> 101

// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/counting-bits
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

pub fn count_bits(n: i32) -> Vec<i32> {
    (0..n+1).map(|i|{
        let b_vec: Vec<char> = format!("{:b}", i).chars().collect();
        b_vec.iter().filter(|e|**e=='1').count() as i32
    }).collect()
}

fn main() {
    println!("{:?}", count_bits(5));
}
