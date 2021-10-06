// 示例 1：

// 输入：[4,2,1]

// 输出：4

// 解释：第一堆力扣币最少需要拿 2 次，第二堆最少需要拿 1 次，第三堆最少需要拿 1 次，总共 4 次即可拿完。

// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/na-ying-bi
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

fn main() {
    let coins = vec!(4,2,1);
    let res = min_count(coins);
    println!("{:?}", res);
}

pub fn min_count(coins: Vec<i32>) -> i32 {
    coins.into_iter().map(|e|e/2+e%2).sum()
}
