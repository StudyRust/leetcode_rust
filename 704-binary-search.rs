pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut left = 0;
    let mut right = nums.len() - 1;
    while left <= right {
        let mid = left + ( right - left ) / 2; // 游标是left作为起点，右侧总长除2，就是右边的中间。
        if nums[mid] == target {
            return mid as i32;
        }
        if nums[mid] > target {
            right = mid - 1; // 如果在左区间，要左移1位，作为开始
        }
        if nums[mid] < target {
            left = mid + 1; // 如果在右区间，要右移1位，作为开始
        }
    }
    -1
}

fn main() {
    let nums = vec![-1,0,3,5,9,12];
    let target = 9;
    println!("{:?}", search(nums, target));
}
