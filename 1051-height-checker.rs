// 示例：

// 输入：heights = [1,1,4,2,1,3]
// 输出：3
// 解释：
// 高度：[1,1,4,2,1,3]
// 预期：[1,1,1,2,3,4]
// 下标 2 、4 、5 处的学生高度不匹配。

// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/height-checker
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

pub fn height_checker(heights: Vec<i32>) -> i32 {
    let mut res = 0;
    let mut sorted_heights = heights.clone();
    sorted_heights.sort();
    for (i, h) in heights.into_iter().enumerate() {
        if h != sorted_heights[i] {
            res += 1;
        }
    }
    res
}
