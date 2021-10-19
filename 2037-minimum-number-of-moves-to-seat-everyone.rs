// 示例 1：

// 输入：seats = [3,1,5], students = [2,7,4]
// 输出：4
// 解释：学生移动方式如下：
// - 第一位学生从位置 2 移动到位置 1 ，移动 1 次。
// - 第二位学生从位置 7 移动到位置 5 ，移动 2 次。
// - 第三位学生从位置 4 移动到位置 3 ，移动 1 次。
// 总共 1 + 2 + 1 = 4 次移动。

// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/minimum-number-of-moves-to-seat-everyone
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

fn main() {
    let seats = vec!(3,1,5);
    let students = vec!(2,7,4);
    println!("{:?}", min_moves_to_seat(seats, students));
}

pub fn min_moves_to_seat(seats: Vec<i32>, students: Vec<i32>) -> i32 {
    let mut sorted_seats = seats.clone();
    sorted_seats.sort();
    let mut sorted_students = students.clone();
    sorted_students.sort();
    let length = seats.len();
    let mut res = 0;
    for i in 0..length {
        res += (sorted_seats[i] - sorted_students[i]).abs();
    }
    res
}
