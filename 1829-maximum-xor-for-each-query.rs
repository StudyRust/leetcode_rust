pub fn get_maximum_xor(nums: Vec<i32>, maximum_bit: i32) -> Vec<i32> {
    let max = 2_i32.pow(maximum_bit as u32) - 1;
    let mut ret = vec![];
    for i in 0..nums.len() {
        let mut tmp = max;
        for num in &nums[0..nums.len() - i] {
            tmp ^= *num;
        }
        ret.push(tmp);
    }
    ret
}

fn main() {
    let nums = vec![2,3,4,7];
    let maximum_bit = 3;
    println!("{:?}", get_maximum_xor(nums, maximum_bit));
}
