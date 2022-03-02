pub fn target_indices(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut sorted_nums = nums.clone();
    sorted_nums.sort();
    let mut sorted_nums_vec: Vec<_> = sorted_nums.iter().enumerate().collect();
    sorted_nums_vec.retain(|e|*e.1==target);
    sorted_nums_vec.iter().map(|e|e.0 as i32).collect()
}

fn main() {
    let nums = vec![1,2,5,2,3];
    let target = 3;
    println!("{:?}", target_indices(nums, target));
}
