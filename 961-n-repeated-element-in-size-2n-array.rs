// solution 1
// pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
//     use std::collections::HashMap;
//     let mut mmp = HashMap::new();
//     let half_len = nums.len() / 2;
//     let mut ret = 0;
//     for num in nums {
//         let count = mmp.entry(num).or_insert(0);
//         *count += 1;
//         if *count == half_len {
//             ret = num;
//             break;
//         }
//     }
//     ret
// }

// solution 2
pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
    let mut sorted_nums = nums.clone();
    sorted_nums.sort();
    let half_len = nums.len() / 2;
    if sorted_nums[half_len] == sorted_nums[half_len+1] { sorted_nums[half_len+1] } else { sorted_nums[half_len-1] }
}

fn main() {
    let nums = vec![3,3,5,9];
    println!("{:?}", repeated_n_times(nums));
}
