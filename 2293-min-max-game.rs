pub fn min_max_game(nums: Vec<i32>) -> i32 {
    let mut nums = nums.clone();
    while nums.len() > 1 {
        let nums_chunks = nums.chunks(2);
        let mut tmp = vec!();
        for (i, chunk) in nums_chunks.enumerate() {
            if i % 2 == 0 {
                tmp.push(*chunk.iter().min().unwrap());
            } else {
                tmp.push(*chunk.iter().max().unwrap());
            }
        }
        nums = tmp;
    }
    nums[0]
}

fn main() {
    let nums = vec![1,3,5,2,4,8,2,2];
    println!("{:?}", min_max_game(nums));
}
