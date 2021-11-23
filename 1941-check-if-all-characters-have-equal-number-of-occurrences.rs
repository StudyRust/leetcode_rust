// 示例 1：

// 输入：s = "abacbc"
// 输出：true
// 解释：s 中出现过的字符为 'a'，'b' 和 'c' 。s 中所有字符均出现 2 次。

fn main() {
	let s = "fhojjkontbncdhwxbnexplclvjyexzsvqyyhpfpnvhdskuhkuoihiqgalklqketjikdlgrawhfo".to_string();
	println!("{:?}", are_occurrences_equal(s));
}

pub fn are_occurrences_equal(s: String) -> bool {
	let mut uniq_chars: Vec<char> = s.chars().collect();
	uniq_chars.sort_unstable();
	uniq_chars.dedup();
	if s.len() % uniq_chars.len() != 0 {
		return false;
	}
	let freq = s.len() / uniq_chars.len();
	for c in uniq_chars {
		if s.chars().filter(|&e| e == c).count() != freq {
			return false;
		}
	}
	return true;
}
