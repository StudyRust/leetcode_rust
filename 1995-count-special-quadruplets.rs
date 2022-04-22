pub fn count_quadruplets(nums: Vec<i32>) -> i32 {
    let mut ret = 0;
    for a in 0..nums.len() {
        for b in (a+1)..nums.len() {
            for c in (b+1)..nums.len() {
                for d in (c+1)..nums.len() {
                    if nums[a] + nums[b] + nums[c] == nums[d] {
                        ret += 1;
                    }
                }
            }
        }
    }
    ret
}

fn main() {
    let nums = vec![1,1,1,3,5];
    println!("{:?}", count_quadruplets(nums));
}
