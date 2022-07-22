pub fn find_min_fibonacci_numbers(k: i32) -> i32 {
    let mut fib_seq = vec![1];
    let mut tmp = 1;
    // 动态规划，列出1000000000以内fib数列
    while tmp < 10_i32.pow(9) {
        fib_seq.push(tmp);
        let len = fib_seq.len();
        tmp = fib_seq[len-1] + fib_seq[len-2];
    }
    let mut ret = 0;
    let mut k = k;
    // 贪心算法
    while k != 0 {
        for i in fib_seq.iter().rev() {
            if k - i >= 0 {
                k -= i;
                ret += 1;
            }
        }
    }
    ret
}

fn main() {
    println!("{:?}", find_min_fibonacci_numbers(7));
}
