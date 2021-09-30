// 示例 1：

// 输入：nums = [-1,-2,-3,-4,3,2,1]
// 输出：1
// 解释：数组中所有值的乘积是 144 ，且 signFunc(144) = 1

// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/sign-of-the-product-of-an-array
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

fn main() {
    let nums = vec!(-1,-2,-3,-4,3,2,1);
    let res = array_sign(nums);
    println!("{:?}", res);
}
// 思路 遍历，如果出现0 返回0，出现正数不变，负数乘以 -1
pub fn array_sign(nums: Vec<i32>) -> i32 {
    let mut res = 1;
    for num in nums {
        if num == 0 {
            res = 0;
            break;
        } else if num < 0 {
            res *= -1;
        }
    }
    res
}
