pub fn is_monotonic(nums: Vec<i32>) -> bool {
    let mut sorted_nums = nums.clone();
    sorted_nums.sort();
    if nums == sorted_nums {
        true
    } else {
        sorted_nums.reverse();
        nums == sorted_nums
    }
}

fn main() {
    let nums = vec![1,2,2,3];
    println!("{:?}", is_monotonic(nums));
}
