// 输入：points = [[1,1],[3,4],[-1,0]]
// 输出：7
// 解释：一条最佳的访问路径是： [1,1] -> [2,2] -> [3,3] -> [3,4] -> [2,3] -> [1,2] -> [0,1] -> [-1,0]
// 从 [1,1] 到 [3,4] 需要 3 秒
// 从 [3,4] 到 [-1,0] 需要 4 秒
// 一共需要 7 秒

// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/minimum-time-visiting-all-points
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

fn main() {
	let points = vec!(
		vec!(1,1),
		vec!(3,4),
		vec!(-1,0)
	);
	println!("{:?}", min_time_to_visit_all_points(points));
}

// 归纳总结题目。经过归纳总结发现，发现2点间取x距离或y距离的较大那个就是时间。
pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
	let mut start = points[0].clone();
	let mut steps = 0;
	for point in points {
		let x_distance = (point[0] - start[0]).abs();
		let y_distance = (point[1] - start[1]).abs();
		if x_distance > y_distance {
			steps += x_distance;
		} else {
			steps += y_distance;
		}
		start = point.clone();
	}
	steps
}
