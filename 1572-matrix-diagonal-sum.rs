// 输入：mat =       [[1,2,3],
//             [4,5,6],
//             [7,8,9]]
// 输出：25
// 解释：对角线的和为：1 + 5 + 9 + 3 + 7 = 25
// 请注意，元素 mat[1][1] = 5 只会被计算一次。
// 示例  2：

// 输入：mat =       [[1,1,1,1],
//             [1,1,1,1],
//             [1,1,1,1],
//             [1,1,1,1]]
// 输出：8
// 示例 3：

// 输入：mat = [[5]]
// 输出：5

// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/matrix-diagonal-sum
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

fn main() {
    let mat = vec!(
        vec!(1,2,3),
        vec!(4,5,6),
        vec!(7,8,9)
    );
    let res = diagonal_sum(mat);
    println!("{:?}", res);
}

pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;
    let length = mat.len();
    for (i, v) in mat.iter().enumerate() {
        sum += v[i];
        sum += v[length-i-1];
    }
    if length % 2 != 0 {
        sum -= mat[length/2][length/2];
    }
    sum
}
