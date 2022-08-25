pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
    let mut ret = vec![];
    let nums_len = nums.len();
    'outer: for i in 0..nums_len {
        for j in i+1..nums_len {
            if nums[j] > nums[i] {
                ret.push(nums[j]);
                continue 'outer;
            }
        }
        for k in 0..i {
            if nums[k] > nums[i] {
                ret.push(nums[k]);
                continue 'outer;
            }
        }
        ret.push(-1);
    }
    ret
}

fn main() {
    let nums = vec![1, 2, 1];
    println!("{:?}", next_greater_elements(nums));
}
