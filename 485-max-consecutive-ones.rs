pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
    nums.split(|e|*e == 0).map(|a|a.len()).max().unwrap() as i32
}

fn main() {
    let nums = vec![1,1,0,1,1,1];
    println!("{:?}", find_max_consecutive_ones(nums));
}
