// 示例 1：

// 输入：target = [1,2,3,4], arr = [2,4,1,3]
// 输出：true
// 解释：你可以按照如下步骤使 arr 变成 target：
// 1- 翻转子数组 [2,4,1] ，arr 变成 [1,4,2,3]
// 2- 翻转子数组 [4,2] ，arr 变成 [1,2,4,3]
// 3- 翻转子数组 [4,3] ，arr 变成 [1,2,3,4]
// 上述方法并不是唯一的，还存在多种将 arr 变成 target 的方法。

// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/make-two-arrays-equal-by-reversing-sub-arrays
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

pub fn can_be_equal(target: Vec<i32>, arr: Vec<i32>) -> bool {
    let mut sorted_target = target.clone();
    let mut sorted_arr = arr.clone();
    sorted_target.sort();
    sorted_arr.sort();
    // println!("{:?} {:?}", sorted_target, sorted_arr);
    sorted_target == sorted_arr
}

fn main() {
    println!("{:?}", can_be_equal(vec!(3,7,9), vec!(3,7,11)));
}
