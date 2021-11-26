// 示例 1：

// 输入：num = 9669
// 输出：9969
// 解释：
// 改变第一位数字可以得到 6669 。
// 改变第二位数字可以得到 9969 。
// 改变第三位数字可以得到 9699 。
// 改变第四位数字可以得到 9666 。
// 其中最大的数字是 9969 。

// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/maximum-69-number
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

pub fn maximum69_number (num: i32) -> i32 {
    let mut flag = true;
    let a: String = num.to_string().chars().map(|d|
        if d != '9' && flag {
            flag = false;
            '9'
        } else {
            d
        }
    ).into_iter().collect();
    a.parse::<i32>().unwrap()
}

fn main() {
    println!("{:?}", maximum69_number(9669));
}
