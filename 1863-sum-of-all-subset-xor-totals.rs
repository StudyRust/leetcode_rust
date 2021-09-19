// 示例 2：

// 输入：nums = [5,1,6]
// 输出：28
// 解释：[5,1,6] 共有 8 个子集：
// - 空子集的异或总和是 0 。
// - [5] 的异或总和为 5 。
// - [1] 的异或总和为 1 。
// - [6] 的异或总和为 6 。
// - [5,1] 的异或总和为 5 XOR 1 = 4 。
// - [5,6] 的异或总和为 5 XOR 6 = 3 。
// - [1,6] 的异或总和为 1 XOR 6 = 7 。
// - [5,1,6] 的异或总和为 5 XOR 1 XOR 6 = 2 。
// 0 + 5 + 1 + 6 + 4 + 3 + 7 + 2 = 28

// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/sum-of-all-subset-xor-totals
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

fn main() {
    let nums = vec!(5,1,6);
    let res = subset_xor_sum(nums);
    println!("{:?}", res);
}

fn powerset(s: Vec<i32>) -> Vec<Vec<i32>> {
    let mut subsets: Vec<Vec<i32>> = vec![];
    let empty: Vec<i32> = vec![];
    subsets.push(empty);

    let mut updated: Vec<Vec<i32>> = vec![]; 

    for ele in s {
        for mut sub in subsets.clone() {
            sub.push(ele);
            updated.push(sub);
        }
        subsets.append(&mut updated);
    }
    subsets
}

pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
    // println!("{:?}", powerset(nums));
    let mut res = 0;
    let subsets = powerset(nums);
    for subset in subsets {
        if subset.len() == 1 {
            res += subset[0];
        } else {
            let mut v = 0;
            for e in subset {
                v = v ^ e;
            }
            res += v;
        }
    }
    res
}