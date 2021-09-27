// 示例 1：

// 输入：nums = [2,5,6,9,10]
// 输出：2
// 解释：
// nums 中最小的数是 2
// nums 中最大的数是 10
// 2 和 10 的最大公约数是 2

// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/find-greatest-common-divisor-of-array
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

fn main() {
    let nums = vec!(2,5,6,9,10);
    let res = find_gcd(nums);
    println!("{:?}", res);
}

pub fn find_gcd(nums: Vec<i32>) -> i32 {
    let min = *nums.iter().min().unwrap();
    let max = *nums.iter().max().unwrap();
    for i in (2..(min+1)).rev() {
        if min % i == 0 && max % i == 0 {
            return i
        }
    }
    return 1
}