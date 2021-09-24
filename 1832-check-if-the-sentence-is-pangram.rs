// 示例 1：

// 输入：sentence = "thequickbrownfoxjumpsoverthelazydog"
// 输出：true
// 解释：sentence 包含英语字母表中每个字母至少一次。

// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/check-if-the-sentence-is-pangram
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

fn main() {
	let sentence = "thequickbrownfoxjumpsoverthelazydog".to_string();
	let res = check_if_pangram(sentence);
	println!("{:?}", res);
}

pub fn check_if_pangram(sentence: String) -> bool {
	let mut chars: Vec<char> = sentence.chars().collect();
	chars.sort_unstable();
	chars.dedup();
	let str: String = chars.into_iter().collect();
	str == "abcdefghijklmnopqrstuvwxyz"
}
