// 示例 2：

// 输入：word1 = "ab", word2 = "pqrs"
// 输出："apbqrs"
// 解释：注意，word2 比 word1 长，"rs" 需要追加到合并后字符串的末尾。
// word1：  a   b
// word2：    p   q   r   s
// 合并后：  a p b q   r   s
// 示例 3：

// 输入：word1 = "abcd", word2 = "pq"
// 输出："apbqcd"
// 解释：注意，word1 比 word2 长，"cd" 需要追加到合并后字符串的末尾。
// word1：  a   b   c   d
// word2：    p   q
// 合并后：  a p b q c   d

// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/merge-strings-alternately
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

pub fn merge_alternately(word1: String, word2: String) -> String {
    let mut res: Vec<char> = vec!();
    let max_len;
    if word1.len() > word2.len() {
        max_len = word1.len();
    } else {
        max_len = word2.len();
    }

    for i in 0..max_len {
        if i < word1.len() {
            res.push(word1.chars().nth(i).unwrap());
        }
        if i < word2.len() {
            res.push(word2.chars().nth(i).unwrap());
        }
    }
    res.into_iter().collect()
}

fn main() {
    println!("{:?}", merge_alternately("abcd".to_string(), "m".to_string()));
}
