// 示例 1：

// 输入：allowed = "ab", words = ["ad","bd","aaab","baa","badab"]
// 输出：2
// 解释：字符串 "aaab" 和 "baa" 都是一致字符串，因为它们只包含字符 'a' 和 'b' 。

// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/count-the-number-of-consistent-strings
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

fn main() {
    let allowed = "ab".to_string();
    let words = ["ad","bd","aaab","baa","badab"].iter().map(|&s|s.into()).collect();
    println!("{:?}", count_consistent_strings(allowed, words));
}

pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
    let allowed_chars: Vec<char> = allowed.chars().collect(); // Chars转换成vector，因为下面使用的contains是vector的方法
    let mut counter = words.len();
    for word in words {
        for c in word.chars() {
            if allowed_chars.contains(&c) {
                continue;
            } else {
                counter -= 1;
                break;
            }
        }
    }
    counter as i32
}