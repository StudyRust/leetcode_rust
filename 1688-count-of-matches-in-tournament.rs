// 示例 1：

// 输入：n = 7
// 输出：6
// 解释：比赛详情：
// - 第 1 轮：队伍数 = 7 ，配对次数 = 3 ，4 支队伍晋级。
// - 第 2 轮：队伍数 = 4 ，配对次数 = 2 ，2 支队伍晋级。
// - 第 3 轮：队伍数 = 2 ，配对次数 = 1 ，决出 1 支获胜队伍。
// 总配对次数 = 3 + 2 + 1 = 6

// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/count-of-matches-in-tournament
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

fn main() {
    println!("{:?}", number_of_matches(7));
}

// 数学归纳法 n-1
pub fn number_of_matches(n: i32) -> i32 {
    n - 1
}
