pub fn array_pair_sum(nums: Vec<i32>) -> i32 {
    let mut sorted_nums = nums.clone();
    sorted_nums.sort();
    let mut res = 0;
    for (i, n) in sorted_nums.iter().enumerate() {
        if i % 2 == 0 {
            res += n;
        }
    }
    res
}

fn main() {
    let nums = vec![1,4,3,2];
    println!("{:?}", array_pair_sum(nums));
}
