use std::time::{SystemTime, UNIX_EPOCH};
struct Solution {
    nums: Vec<i32>
}
impl Solution {
    fn new(nums: Vec<i32>) -> Self {
        Self { nums }
    }
    fn reset(&self) -> Vec<i32> {
        self.nums.clone()
    }
    fn shuffle(&self) -> Vec<i32> {
        let mut nums = self.nums.clone();
        let mut n = nums.len() as u128;
        while n > 1 {
            let m = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() % n;
            nums.swap(m as usize, n as usize - 1);
            n -= 1;
        }
        nums
    }
}

fn main() {
    let nums = vec![1, 2, 3];
    let obj = Solution::new(nums);
    let ret_1: Vec<i32> = obj.reset();
    println!("{:?}", ret_1);
    let ret_2: Vec<i32> = obj.shuffle();
    println!("{:?}", ret_2);
    let ret_3: Vec<i32> = obj.reset();
    println!("{:?}", ret_3);
}
