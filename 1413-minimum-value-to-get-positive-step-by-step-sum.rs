pub fn min_start_value(nums: Vec<i32>) -> i32 {
    let mut start_value = 1;
    'outer: loop {
        let mut tmp_sum = start_value;
        for num in &nums {
            tmp_sum += num;
            if tmp_sum < 1 {
                start_value += 1;
                continue 'outer;
            }
        }
        break;
    }
    start_value
}

fn main() {
    let nums = vec![1,-2,-3];
    println!("{:?}", min_start_value(nums));
}
