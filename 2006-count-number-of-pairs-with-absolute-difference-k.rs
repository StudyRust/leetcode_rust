// 输入：nums = [3,2,1,5,4], k = 2
// 输出：3
// 解释：差的绝对值为 2 的数对为：
// - 3 1
// - 3 5
// - 2 4

// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/count-number-of-pairs-with-absolute-difference-k
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

fn main() {
    let nums = vec!(3,2,1,5,4);
    let k = 2;
    let res = count_k_difference(nums, k);
    println!("{:?}", res);
}

pub fn count_k_difference(nums: Vec<i32>, k: i32) -> i32 {
    let nums_len = &nums.len();
    let mut counter = 0;
    for (i, num) in nums.iter().enumerate() {
        for n in &nums[i+1..*nums_len] {
            if (num - n).abs() == k {
                counter += 1;
            }
        }
    }
    counter
}
