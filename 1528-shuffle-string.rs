// 输入：s = "codeleet", indices = [4,5,6,7,0,2,1,3]
// 输出："leetcode"
// 解释：如图所示，"codeleet" 重新排列后变为 "leetcode" 。

// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/shuffle-string
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

pub fn restore_string(s: String, indices: Vec<i32>) -> String {
    let mut res: Vec<char> = vec!['c'; s.len()];
    for (i, c) in s.chars().enumerate() {
        res[indices[i as usize] as usize] = c;
    }
    res.into_iter().collect()
}

fn main() {
    let s = "codeleet".to_string();
    let indices = vec!(4,5,6,7,0,2,1,3);
    println!("{:?}", restore_string(s, indices));
}
