// 示例 1：

// 输入：n = 34, k = 6
// 输出：9
// 解释：34 (10 进制) 在 6 进制下表示为 54 。5 + 4 = 9 。
// 示例 2：

// 输入：n = 10, k = 10
// 输出：1
// 解释：n 本身就是 10 进制。 1 + 0 = 1 。

// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/sum-of-digits-in-base-k
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。


// 先分析，因为1 <= n <= 100 所以按2进制算最多7位数
pub fn sum_base(n: i32, k: i32) -> i32 {
    let mut res = 0;
    let mut temp = n;
    for i in (0..7).rev() {
        let p = k.pow(i);
        let r = temp / p;
        res += r;
        temp -= r * p;
    }
    res
}

fn main() {
    println!("{:?}", sum_base(34, 6));
}
