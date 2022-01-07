pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    let mut res: Vec<i32> = nums.iter().map(|e|e*e).collect();
    res.sort();
    res
}

fn main() {
    let nums = vec![-4,-1,0,3,10];
    println!("{:?}", sorted_squares(nums));
}
