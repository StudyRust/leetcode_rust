fn main() {
	let nums = vec!(0,2,1,5,3,4);
	let res = build_array(nums);
	println!("{:?}", res);
}

// 输入：nums = [0,2,1,5,3,4]
// 输出：[0,1,2,4,5,3]
// 解释：数组 ans 构建如下：
// ans = [nums[nums[0]], nums[nums[1]], nums[nums[2]], nums[nums[3]], nums[nums[4]], nums[nums[5]]]
//     = [nums[0], nums[2], nums[1], nums[5], nums[3], nums[4]]
//     = [0,1,2,4,5,3]

pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
	nums.clone().into_iter().map(|x| nums[x as usize]).collect()
}