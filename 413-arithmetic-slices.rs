pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
    if nums.len() < 3 { return 0; }
    pub fn is_arithmetic(nums: &[i32]) -> bool {
        let minus = nums[1] - nums[0];
        for i in 2..nums.len() {
            if nums[i] - nums[i-1] != minus { return false; }
        }
        true
    }
    let mut ret = 0;
    for length in 3..=nums.len() {
        for i in 0..=nums.len() - length {
            if is_arithmetic(&nums[i..i+length]) { ret += 1; }
        }
    }
    ret
}

fn main() {
    println!("{:?}", number_of_arithmetic_slices(vec![1, 2, 3, 4]));
}
