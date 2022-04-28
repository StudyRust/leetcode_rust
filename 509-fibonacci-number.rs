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

    pub fn fib_dp(n: i32) -> i32 {
        let mut dp = vec![0; 31];
        dp[1] = 1;
        for i in 2..=30 {
            dp[i] = dp[i-1] + dp[i-2];
        }
        dp[n as usize]
    }
}

fn main() {
    println!("{:?}", Solution::fib(6));
    println!("{:?}", Solution::fib_dp(6));
}
