// 经典递归（两个条件：）
// 1. 有出口 n=1 n=2
// 2. 有公式 fib(n) = fib(n-1) + fib(n-2)

struct Solution;

impl Solution {
    pub fn fib(n: i32) -> i32 {
        if n == 0 {
            0
        } else if n == 1 {
            1
        } else {
            Self::fib(n-1) + Self::fib(n-2)
        }
    }
}

fn main() {
    println!("{:?}", Solution::fib(3));
}
