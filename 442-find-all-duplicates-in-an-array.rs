struct Solution;

impl Solution {
    pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;
        let mut mmp = HashMap::new();
        for num in nums {
            let count = mmp.entry(num).or_insert(0);
            *count += 1;
        }
        mmp.retain(|_, v|v > &mut 1);
        mmp.into_keys().collect::<Vec<i32>>()
    }
}

fn main() {
    let nums = vec![4,3,2,7,8,2,3,1];
    println!("{:?}", Solution::find_duplicates(nums));
}
