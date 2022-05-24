pub fn special_array(nums: Vec<i32>) -> i32 {
    for x in 0..=nums.len() {
        let x = x as i32;
        let mut count = 0;
        for num in &nums {
            if *num >= x {
                count += 1;
            }
        }
        if count == x {
            return x;
        }
    }
    -1
}

fn main() {
    let nums = vec![3,5];
    println!("{:?}", special_array(nums));
}
