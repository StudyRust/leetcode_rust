// 示例 1：

// 输入：arr = [1,4,2,5,3]
// 输出：58
// 解释：所有奇数长度子数组和它们的和为：
// [1] = 1
// [4] = 4
// [2] = 2
// [5] = 5
// [3] = 3
// [1,4,2] = 7
// [4,2,5] = 11
// [2,5,3] = 10
// [1,4,2,5,3] = 15
// 我们将所有值求和得到 1 + 4 + 2 + 5 + 3 + 7 + 11 + 10 + 15 = 58

// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/sum-of-all-odd-length-subarrays
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。


// 移动窗口，窗口为 1 窗口为3 窗口为5
fn main() {
    let arr = vec!(1,4,2,5,3);
    println!("{:?}", sum_odd_length_subarrays(arr));
}

pub fn sum_odd_length_subarrays(arr: Vec<i32>) -> i32 {
    let n = &arr.len();
    let mut sum = 0;
    for i in 0..(n/2+1) {
        let window_length = 2*i+1;
        for i in 0..*n {
            if i+window_length <= *n {
                for j in &arr[i..i+window_length] {
                    sum += j;
                }
            }
        }
    }
    sum
}
