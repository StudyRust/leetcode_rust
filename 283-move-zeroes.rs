// 跟紧接着不为0的数字互换
pub fn move_zeroes(nums: &mut Vec<i32>) {
    for i in 0..nums.len() {
        for j in i..nums.len() {
            if nums[i] == 0 && nums[j] != 0 { nums.swap(i, j) }
        }
    }
}

fn main() {
    let mut nums = vec![1, 0, 0, 4, 3, 12];
    move_zeroes(&mut nums);
    println!("{:?}", nums);
    let mut nums = vec![1, 0, 0, 0, 1];
    move_zeroes(&mut nums);
    println!("{:?}", nums);
}
