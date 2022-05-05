struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut ret = vec!(0; nums.len());
        let mut left = vec!(0; nums.len());
        let mut right = vec!(0; nums.len());
        left[0] = 1;
        right[nums.len()-1] = 1;
        for i in 1..nums.len() {
            left[i] = nums[i-1] * left[i-1];
        }
        println!("{:?}", left);
        for j in (0..(nums.len()-1)).rev() {
            right[j] = nums[j+1] * right[j+1];
        }
        println!("{:?}", right);
        for i in 0..nums.len() {
            ret[i] = left[i] * right[i];
        }
        ret
    }
}

fn main() {
    let nums = vec![1,2,3,4];
    println!("{:?}", Solution::product_except_self(nums));
}
