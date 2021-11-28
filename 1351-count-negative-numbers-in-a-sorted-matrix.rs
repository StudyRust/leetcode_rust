// 示例 1：

// 输入：grid = [[4,3,2,-1],[3,2,1,-1],[1,1,-1,-2],[-1,-1,-2,-3]]
// 输出：8
// 解释：矩阵中共有 8 个负数。

// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/count-negative-numbers-in-a-sorted-matrix
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
    grid.iter().map(|arr|
        arr.iter().map(|e|
            if *e < 0 {
                1
            } else {
                0
            }
        ).sum::<i32>()
    ).sum()
}

fn main() {
    let grid = vec!(
        vec!(4,3,2,-1),
        vec!(3,2,1,-1),
        vec!(1,1,-1,-2),
        vec!(-1,-1,-2,-3)
    );
    println!("{:?}", count_negatives(grid));
}
