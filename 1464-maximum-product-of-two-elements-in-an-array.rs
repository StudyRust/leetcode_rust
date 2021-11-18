// 示例 1：

// 输入：nums = [3,4,5,2]
// 输出：12
// 解释：如果选择下标 i=1 和 j=2（下标从 0 开始），则可以获得最大值，(nums[1]-1)*(nums[2]-1) = (4-1)*(5-1) = 3*4 = 12 。 

// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/maximum-product-of-two-elements-in-an-array
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

pub fn max_product(nums: Vec<i32>) -> i32 {
    let length = nums.len();
    let mut sorted = nums.clone();
    sorted.sort();
    (sorted[length-1]-1) * (sorted[length-2]-1)
}
