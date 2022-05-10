use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut set = HashSet::new();
        let mut n = n;
        let mut new_n: i32;
        while !set.contains(&n) {
            set.insert(n);
            new_n = 0;
            while n > 0 {
                new_n += (n % 10).pow(2);
                n /= 10;
            }
            n = new_n;
        }
        n == 1
    }
}

fn main() {
    println!("{:?}", Solution::is_happy(7));
}
