// 示例 1：

// 输入：s = "book"
// 输出：true
// 解释：a = "bo" 且 b = "ok" 。a 中有 1 个元音, b 也有 1 个元音。所以, a 和 b 相似。

// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/determine-if-string-halves-are-alike
// 著作权归领扣网络所有。商业转载请联系官方授权, 非商业转载请注明出处。

pub fn halves_are_alike(s: String) -> bool {
    fn count_meta(chars: Vec<char>) -> i32 {
        let metas = vec!('a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U');
        chars.iter().map(|c|
            if metas.contains(c) {
                1
            } else {
                0
            }
        ).sum()
    }
    let mutschars: Vec<char> = s.chars().collect();
    let length = mutschars.len();
    count_meta(mutschars[0..length/2].to_vec()) == count_meta(mutschars[length/2..].to_vec())
}

fn main() {
    println!("{:?}", halves_are_alike("textbook".to_string()));
}
