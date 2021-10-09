// 示例 1：

// 输入：s = "a1c1e1"
// 输出："abcdef"
// 解释：数字被替换结果如下：
// - s[1] -> shift('a',1) = 'b'
// - s[3] -> shift('c',1) = 'd'
// - s[5] -> shift('e',1) = 'f'

// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/replace-all-digits-with-characters
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

fn main() {
	let s = "a1c1e1".to_string();
	println!("{:?}", replace_digits(s));
}

pub fn replace_digits(s: String) -> String {
	let chars = s.chars().clone();
	let mut res = "".to_string();
	let mut front_char = 'a';
	for c in chars {
		if c.is_digit(10) {
			res.push(std::char::from_u32(front_char as u32 + (c as u32 - 0x30)).unwrap());
		} else {
			front_char = c;
			res.push(c);
		}
	}
	res
}
