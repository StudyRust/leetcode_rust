// 示例 1：

// 输入：nums = [5,6,2,7,4]
// 输出：34
// 解释：可以选出下标为 1 和 3 的元素构成第一个数对 (6, 7) 以及下标 2 和 4 构成第二个数对 (2, 4)
// 乘积差是 (6 * 7) - (2 * 4) = 34

// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/maximum-product-difference-between-two-pairs
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

fn main() {
	let nums = vec!(5,6,2,7,4);
	let res = max_product_difference(nums);
	println!("{:?}", res);
}

pub fn max_product_difference(nums: Vec<i32>) -> i32 {
	let mut sorted_nums = nums.clone();
	sorted_nums.sort();
	let length = nums.len();
	sorted_nums[length-1] * sorted_nums[length-2] - sorted_nums[0] * sorted_nums[1]
}
