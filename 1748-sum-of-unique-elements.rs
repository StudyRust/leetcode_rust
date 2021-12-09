// 示例 1：

// 输入：nums = [1,2,3,2]
// 输出：4
// 解释：唯一元素为 [1,3] ，和为 4 。

pub fn sum_of_unique(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let mut kvm = HashMap::new();
    for num in nums {
        if !kvm.contains_key(&num) {
            kvm.insert(num, 1);
        } else {
            *kvm.get_mut(&num).unwrap() += 1;
        }
    }
    kvm.retain(|_, v| *v == 1 );
    kvm.keys().sum::<i32>()
}

fn main() {
    println!("{:?}", sum_of_unique(vec!(1,2,3,2)));
}
