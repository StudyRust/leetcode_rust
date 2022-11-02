pub fn average_value(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.retain(|num| { num % 6 == 0 });
    (nums.iter().sum::<i32>() as f64 / nums.len() as f64).floor() as i32
}

fn main() {
    println!("{:?}", average_value(vec![1,3,6,10,12,15]));
    println!("{:?}", average_value(vec![1,2,4,7,10]));
}
