fn main() {
	let accounts = vec!(vec!(1, 2, 3), vec!(3, 2, 1));
	let a = maximum_wealth(accounts);
	println!("{:?}", a);
}

// 输入：accounts = [[1,2,3],[3,2,1]]
// 输出：6
// 解释：
// 第 1 位客户的资产总量 = 1 + 2 + 3 = 6
// 第 2 位客户的资产总量 = 3 + 2 + 1 = 6
// 两位客户都是最富有的，资产总量都是 6 ，所以返回 6 。

// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/richest-customer-wealth
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
	let mut res = 0;
	for account in accounts {
		let mut current = 0;
		for e in account {
			current += e;
		}
		if current >= res {
			res = current
		}
	}
	res
}