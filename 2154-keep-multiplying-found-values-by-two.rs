pub fn find_final_value(nums: Vec<i32>, original: i32) -> i32 {
    let mut tmp = original;
    loop {
        if nums.iter().any(|x|*x==tmp) {
            tmp *= 2;
            continue;
        } else {
            break;
        }
    }
    tmp
}

fn main() {
    let nums = vec![5,3,6,1,12];
    let original = 3;
    println!("{:?}", find_final_value(nums, original));
}
