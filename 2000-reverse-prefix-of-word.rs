// 示例 1：

// 输入：word = "abcdefd", ch = "d"
// 输出："dcbaefd"
// 解释："d" 第一次出现在下标 3 。
// 反转从下标 0 到下标 3（含下标 3）的这段字符，结果字符串是 "dcbaefd" 。

// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/reverse-prefix-of-word
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

fn main() {
    let word = "abcdefd".to_string();
    let ch = 'd';
    println!("{:?}", reverse_prefix(word, ch));
}

pub fn reverse_prefix(word: String, ch: char) -> String {
    let s_index = word.find(ch);
    match s_index {
        Some(p) => {
            let res = word.clone();
            let mut left = res[..p+1].chars().rev().collect::<String>();
            let right: String = res[p+1..].to_string();
            left.push_str(&right);
            left
        },
        None => word,
    }
}
