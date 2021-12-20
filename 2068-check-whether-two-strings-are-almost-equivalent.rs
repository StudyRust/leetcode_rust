// 示例 1：

// 输入：word1 = "aaaa", word2 = "bccb"
// 输出：false
// 解释：字符串 "aaaa" 中有 4 个 'a' ，但是 "bccb" 中有 0 个 'a' 。
// 两者之差为 4 ，大于上限 3 。

// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/check-whether-two-strings-are-almost-equivalent
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

pub fn check_almost_equivalent(word1: String, word2: String) -> bool {
    use std::collections::HashMap;
    let mut word_map = HashMap::new();
    for c in word1.chars() {
        if !word_map.contains_key(&c) {
            word_map.insert(c, 1);
        } else {
            *word_map.get_mut(&c).unwrap() += 1;
        }
    }
    for c in word2.chars() {
        if !word_map.contains_key(&c) {
            word_map.insert(c, -1);
        } else {
            *word_map.get_mut(&c).unwrap() -= 1;
        }
    }
    word_map.retain(|_, v|*v > 3 || *v < -3);
    word_map.is_empty()
}

fn main() {
    println!("{:?}", check_almost_equivalent("aaaa".to_string(), "bccb".to_string()));
}
