pub fn count_elements(nums: Vec<i32>) -> i32 {
    let min = *nums.iter().min().unwrap();
    let max = *nums.iter().max().unwrap();
    let mut ret = 0;
    for num in nums {
        if num == min { continue }
        if num == max { continue }
        ret += 1;
    }
    ret
}

fn main() {
    let nums = vec![-65,-65,50,-65,50,-55,-65,-65];
    println!("{:?}", count_elements(nums));
}
