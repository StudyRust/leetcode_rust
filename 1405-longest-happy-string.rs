// 贪心：每次都选择剩下数量最多的字母添加 ，因为 数量少的字母需要用做中间隔着，这样才会最长。全局最优由局部最优构成

pub fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
    let mut ret = String::new();
    let (mut a, mut b, mut c) = (a, b, c);
    let mut pre: _;
    ret
}

fn main() {
    println!("{:?}", longest_diverse_string(1, 1, 7));
}
