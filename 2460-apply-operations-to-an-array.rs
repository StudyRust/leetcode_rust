pub fn apply_operations(nums: Vec<i32>) -> Vec<i32> {
    let mut none_zero = vec![];
    let mut zero = vec![];
    let mut nums_clone = nums.clone();
    for i in 0 .. nums.len() {
        if nums_clone[i] == 0 {
            zero.push(0);
            continue
        }
        if i + 1 < nums.len() && nums[i] == nums[i + 1] {
            nums_clone[i] *= 2;
            nums_clone[i + 1] = 0
        }
        none_zero.push(nums_clone[i]);
    }
    [none_zero, zero].concat()
}

fn main() {
    println!("{:?}", apply_operations(vec![1, 2, 2, 1, 1, 0]));
    println!("{:?}", apply_operations(vec![847,847,0,0,0,399,416,416,879,879,206,206,206,272]));
    println!("{:?}", apply_operations(vec![0, 1]));

}
