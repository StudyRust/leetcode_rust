fn main() {
    let nums = vec!(1,2,3);
    let res = num_identical_pairs(nums);
    println!("{:?}", res);
}


// 输入：nums = [1,2,3,1,1,3]
// 输出：4
// 解释：有 4 组好数对，分别是 (0,3), (0,4), (3,4), (2,5) ，下标从 0 开始

// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/number-of-good-pairs
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
// [1,1,1,1] 6
// [1,2,3]   0


pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
    let mut sum = 0;
    let length = &nums.len();                       // 这里为什么用& 因为nums(ptr/len/capacity)是immut的 所以只能借用 length是nums.len的借用
    for (i, n) in nums.iter().enumerate() {
        for num in &nums[i+1..*length] {            // 这里为什么用&和* 同理，需要借用nums的slice 并取得length引用的值
            if num == n {
                sum += 1;
            }
        }
    }
    sum
}