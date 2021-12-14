// 示例 1：

// 输入：num = 5
// 输出：2
// 解释：5 的二进制表示为 101（没有前导零位），其补数为 010。所以你需要输出 2 。

pub fn find_complement(num: i32) -> i32 {
    let mut binary_str: String = format!("{:b}", num);
    binary_str = binary_str.replace("0", "-");
    binary_str = binary_str.replace("1", "0");
    binary_str = binary_str.replace("-", "1");
    isize::from_str_radix(&binary_str, 2).unwrap() as i32
}

fn main() {
    println!("{:?}", find_complement(5));
}
