// 输入：operations = ["--X","X++","X++"]
// 输出：1
// 解释：操作按下述步骤执行：
// 最初，X = 0
// --X：X 减 1 ，X =  0 - 1 = -1
// X++：X 加 1 ，X = -1 + 1 =  0
// X++：X 加 1 ，X =  0 + 1 =  1

// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/final-value-of-variable-after-performing-operations
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

fn main() {
    let operations = ["--X","X++","X++"].iter().map(|&s|s.into()).collect();
    let res = final_value_after_operations(operations);
    println!("{:?}", res);
}

pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
    operations.into_iter().map(|s|
        if s.contains('+') {
            1
        } else {
            -1
        }
    ).sum() // 因为map完，vec里面已经是简单type i32了所以不需要 .collect().iter().sum()
}
