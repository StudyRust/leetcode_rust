// 给定一个非空整数数组，除了某个元素只出现一次以外，其余每个元素均出现两次。找出那个只出现了一次的元素。

// 说明：

// 你的算法应该具有线性时间复杂度。 你可以不使用额外空间来实现吗？

// 示例 1:

// 输入: [2,2,1]
// 输出: 1

// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/single-number
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

pub fn single_number(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let mut kvm = HashMap::new();
    for num in nums {
        if !kvm.contains_key(&num) {
            kvm.insert(num, true);
        } else {
            kvm.remove(&num);
        }
    }
    *kvm.keys().nth(0).unwrap()
}


fn main() {
    println!("{:?}", single_number(vec!(2,2,1)));
}