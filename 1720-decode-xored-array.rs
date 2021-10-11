// 示例 1：

// 输入：encoded = [1,2,3], first = 1
// 输出：[1,0,2,1]
// 解释：若 arr = [1,0,2,1] ，那么 first = 1 且 encoded = [1 XOR 0, 0 XOR 2, 2 XOR 1] = [1,2,3]

// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/decode-xored-array
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

fn main() {
	println!("{:?}", decode(vec!(1,2,3), 1));
}

pub fn decode(encoded: Vec<i32>, first: i32) -> Vec<i32> {
	let mut res = vec!(first);
	let mut temp = first;
	for num in encoded {
		temp = temp ^ num;
		res.push(temp);
	}
	res
}
