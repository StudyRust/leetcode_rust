pub fn find_middle_index(nums: Vec<i32>) -> i32 {
    for i in 0..nums.len() {
        if nums[0..i].iter().sum::<i32>() == nums[i+1..].iter().sum::<i32>() {
            return i as i32;
        }
    }
    -1
}

fn main() {
    let nums = vec![1];
    println!("{:?}", find_middle_index(nums));
}
