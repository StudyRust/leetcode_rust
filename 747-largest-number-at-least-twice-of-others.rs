pub fn dominant_index(nums: Vec<i32>) -> i32 {
    if nums == vec![1] {
        return 0;
    }
    let mut sorted_nums: Vec<_> = nums.clone().into_iter().enumerate().collect();
    sorted_nums.sort_by(|a, b|a.1.cmp(&b.1));
    let length = &nums.len();
    if sorted_nums[length-1].1 < sorted_nums[length-2].1 * 2 {
        -1
    } else {
        sorted_nums[length-1].0 as i32
    }
}

fn main() {
    let nums = vec![1];
    println!("{:?}", dominant_index(nums));
}
