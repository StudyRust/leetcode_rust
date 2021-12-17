// 示例 1：

// 输入：arr = [3,5,1]
// 输出：true
// 解释：对数组重新排序得到 [1,3,5] 或者 [5,3,1] ，任意相邻两项的差分别为 2 或 -2 ，可以形成等差数列。

// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/can-make-arithmetic-progression-from-sequence
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

pub fn can_make_arithmetic_progression(arr: Vec<i32>) -> bool {
    let mut sorted_arr = arr.clone();
    sorted_arr.sort();
    for (i, num) in sorted_arr.iter().enumerate() {
        if sorted_arr.get(i+1) != None {
            if num - sorted_arr[i+1] != sorted_arr[0] - sorted_arr[1] {
                return false;
            }
        }
    }
    true
}

fn main() {
    println!("{:?}", can_make_arithmetic_progression(vec![3,5,1]));
}
