// 示例 1：

// 输入：nums = [2,5,1,3,4,7], n = 3
// 输出：[2,3,5,4,1,7]
// 解释：由于 x1=2, x2=5, x3=1, y1=3, y2=4, y3=7 ，所以答案为 [2,3,5,4,1,7]

// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/shuffle-the-array
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

fn main() {
    let nums = vec!(2,5,1,3,4,7);
    let res = shuffle(nums, 3);
    println!("{:?}", res);
}

pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
    let mut res = vec!();
    for i in 0..n {
        res.push(nums[i as usize]);
        res.push(nums[((i as i32)+n) as usize]);
    }
    res
}
