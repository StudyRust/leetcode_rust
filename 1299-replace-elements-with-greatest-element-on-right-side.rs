// 示例 1：

// 输入：arr = [17,18,5,4,6,1]
// 输出：[18,6,6,6,1,-1]
// 解释：
// - 下标 0 的元素 --> 右侧最大元素是下标 1 的元素 (18)
// - 下标 1 的元素 --> 右侧最大元素是下标 4 的元素 (6)
// - 下标 2 的元素 --> 右侧最大元素是下标 4 的元素 (6)
// - 下标 3 的元素 --> 右侧最大元素是下标 4 的元素 (6)
// - 下标 4 的元素 --> 右侧最大元素是下标 5 的元素 (1)
// - 下标 5 的元素 --> 右侧没有其他元素，替换为 -1

// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/replace-elements-with-greatest-element-on-right-side
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

pub fn replace_elements(arr: Vec<i32>) -> Vec<i32> {
    arr.iter().enumerate().map(|(i, _)|{
        let max_value = &arr[i+1..].iter().max();
        match max_value {
            Some(i) => **i as i32,
            None => -1 as i32,
        }
    }).collect()
}

fn main() {
    let arr = vec![17,18,5,4,6,1];
    println!("{:?}", replace_elements(arr));
}
