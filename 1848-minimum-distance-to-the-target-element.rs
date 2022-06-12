pub fn get_min_distance(nums: Vec<i32>, target: i32, start: i32) -> i32 {
    let mut ret = 10000;
    for (i, e) in nums.iter().enumerate() {
        if *e == target {
            let tmp = (i as i32-start).abs();
            if tmp < ret {
                ret = tmp;
            }
        }
    }
    ret
}

fn main() {
    let nums = vec![1,2,3,4,5];
    let target = 5;
    let start = 3;
    println!("{:?}", get_min_distance(nums, target, start));
}
