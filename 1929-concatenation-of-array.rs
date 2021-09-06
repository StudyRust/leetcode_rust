fn main() {
    let nums = vec!(1, 2, 1);
    let a = get_concatenation(nums);
    println!("{:?}", a);
}

// 1929 https://leetcode-cn.com/problems/concatenation-of-array/
// [1,2,1] [1,2,1,1,2,1]
// 审题，函数输入已经固定是 immut vec，那么结果不能用他返回
// 所以需要declare 2 个mut 的nums
pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
    let mut res_nums = nums.clone();
    let mut clone_nums = nums.clone();
    res_nums.append(&mut clone_nums); // 只能 append mut 的nums
    res_nums
}