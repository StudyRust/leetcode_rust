pub fn pivot_index(nums: Vec<i32>) -> i32 {
    let mindex = nums.len();
    for i in 0..mindex {
        if &nums[0..i].iter().sum::<i32>() == &nums[(i+1)..].iter().sum::<i32>() {
            return i as i32;
        }
    }
    -1
}

fn main() {
    let nums = vec![2,1,-1];
    println!("{:?}", pivot_index(nums));
}
