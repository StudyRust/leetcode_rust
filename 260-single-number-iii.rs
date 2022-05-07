struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;
        let mut mmp = HashMap::new();
        for num in nums {
            let count = mmp.entry(num).or_insert(0);
            *count += 1;
        }
        mmp.retain(|_, v|*v==1);
        mmp.into_keys().collect::<Vec<i32>>()
    }
}

fn main() {
    let nums = vec![1,2,1,3,2,5];
    println!("{:?}", Solution::single_number(nums));
}
