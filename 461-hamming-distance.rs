// 两个整数之间的 汉明距离 指的是这两个数字对应二进制位不同的位置的数目。

// 给你两个整数 x 和 y，计算并返回它们之间的汉明距离。

// 示例 1：

// 输入：x = 1, y = 4
// 输出：2
// 解释：
// 1   (0 0 0 1)
// 4   (0 1 0 0)
//        ↑   ↑
// 上面的箭头指出了对应二进制位不同的位置。

// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/hamming-distance
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

fn main() {
    println!("{:?}", hamming_distance(1, 3));
}

pub fn hamming_distance(x: i32, y: i32) -> i32 {
    let x = format!("{:#064b}", x);
    let y = format!("{:#064b}", y);
    let x_chars: Vec<char> = x.chars().collect(); // i32 转 2进制string 转 Vec<char> 才好处理
    let y_chars: Vec<char> = y.chars().collect();
    let mut counter = 0;
    for i in 0..64 {
        if x_chars[i] != y_chars[i] {
            counter += 1;
        }
    }
    counter
}
