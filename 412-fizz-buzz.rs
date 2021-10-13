// 给你一个整数 n ，找出从 1 到 n 各个整数的 Fizz Buzz 表示，并用字符串数组 answer（下标从 1 开始）返回结果，其中：

// answer[i] == "FizzBuzz" 如果 i 同时是 3 和 5 的倍数。
// answer[i] == "Fizz" 如果 i 是 3 的倍数。
// answer[i] == "Buzz" 如果 i 是 5 的倍数。
// answer[i] == i 如果上述条件全不满足。
//  

// 示例 1：

// 输入：n = 3
// 输出：["1","2","Fizz"]

// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/fizz-buzz
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

fn main() {
	println!("{:?}", fizz_buzz(3));
}

pub fn fizz_buzz(n: i32) -> Vec<String> {
	let mut res = vec!();
	for i in 1..n+1 {
		if i % 15 == 0 {
			res.push("FizzBuzz".to_string());
		} else if i % 3 == 0 {
			res.push("Fizz".to_string());
		} else if i % 5 == 0 {
			res.push("Buzz".to_string());
		} else {
			res.push(i.to_string());
		}
	}
	res
}
