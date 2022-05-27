pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
    use std::collections::HashMap;
    let mut mmp = HashMap::new();
    for i in 1..=nums.len() {
        mmp.entry(i as i32).or_insert(0);
    }
    for i in nums {
        let count = mmp.entry(i).or_insert(0);
        *count += 1;
    }
    mmp.retain(|_, v|*v==0);
    mmp.into_keys().collect::<Vec<i32>>()
}

fn main() {
    let nums = vec![4,3,2,7,8,2,3,1];
    println!("{:?}", find_disappeared_numbers(nums));
}
