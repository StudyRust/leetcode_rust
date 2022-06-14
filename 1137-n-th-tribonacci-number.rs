struct Solution;


// 超时
// impl Solution {
//     pub fn tribonacci(n: i32) -> i32 {
//         if n == 0 { return 0; }
//         if n == 1 { return 1; }
//         if n == 2 { return 1; }
//         Self::tribonacci(n-1) + Self::tribonacci(n-2) + Self::tribonacci(n-3)
//     }
// }

// 动态规划
impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        let mut ret = vec![0; 38];
        ret[0] = 0;
        ret[1] = 1;
        ret[2] = 1;
        for i in 3..38 {
            ret[i] = ret[i-1] + ret[i-2] + ret[i-3];
        }
        ret[n as usize]
    }
}

fn main() {
    println!("{:?}", Solution::tribonacci(35));
}
