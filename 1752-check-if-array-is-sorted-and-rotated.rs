
// 统计降序次数
// 最后1位和第一位也要比较，因为是环
// 降序次数不大于1

pub fn check(nums: Vec<i32>) -> bool {
    let mut decrease_count = 0;
    let mut tmp = -1;
    for num in &nums {
        if num < &tmp {
            decrease_count += 1;
        }
        tmp = *num;
    }
    if nums.last().unwrap() > nums.first().unwrap() {
        decrease_count += 1;
    }

    decrease_count <= 1
}

fn main() {
    let nums = vec![6, 2, 6];
    println!("{:?}", check(nums));
}
