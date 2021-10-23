// 示例 1：

// 输入：startTime = [1,2,3], endTime = [3,2,7], queryTime = 4
// 输出：1
// 解释：一共有 3 名学生。
// 第一名学生在时间 1 开始写作业，并于时间 3 完成作业，在时间 4 没有处于做作业的状态。
// 第二名学生在时间 2 开始写作业，并于时间 2 完成作业，在时间 4 没有处于做作业的状态。
// 第三名学生在时间 3 开始写作业，预计于时间 7 完成作业，这是是唯一一名在时间 4 时正在做作业的学生。

// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/number-of-students-doing-homework-at-a-given-time
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

fn main() {
    let start_time = vec!(1,2,3);
    let end_time = vec!(3,2,7);
    let query_time = 4;
    println!("{:?}", busy_student(start_time, end_time, query_time));
}

pub fn busy_student(start_time: Vec<i32>, end_time: Vec<i32>, query_time: i32) -> i32 {
    let length = start_time.len();
    let mut counter = 0;
    for i in 0..length {
        if start_time[i] <= query_time && query_time <= end_time[i] {
            counter += 1;
        }
    }
    counter
}
