// 示例 1：

// 输入：nums = [1,1,1]
// 输出：3
// 解释：你可以进行如下操作：
// 1) 增加 nums[2] ，数组变为 [1,1,2] 。
// 2) 增加 nums[1] ，数组变为 [1,2,2] 。
// 3) 增加 nums[2] ，数组变为 [1,2,3] 。
// 示例 2：

// 输入：nums = [1,5,2,4,1]
// 输出：14
// 示例 3：

// 输入：nums = [8]
// 输出：0

// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/minimum-operations-to-make-the-array-increasing
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

fn main() {
    let nums = vec!(1,5,2,4,1);
    println!("{:?}", min_operations(nums));
}

pub fn min_operations(nums: Vec<i32>) -> i32 {
    let mut temp = 0;
    let mut steps = 0;
    for num in nums {
        if num <= temp {
            steps += temp - num;
            temp += 1;
        } else {
            temp = num + 1;
        }
    }
    steps
}
