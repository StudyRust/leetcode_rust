pub fn sort_colors(nums: &mut Vec<i32>) {
    let lst_idx = nums.len() - 1;
    for i in 0..lst_idx {
        for j in 0..lst_idx-i {
            if nums[j] > nums[j+1] {
                let tmp = nums[j];
                nums[j] = nums[j+1];
                nums[j+1] = tmp;
            }
        }
    }
}

fn main() {
    let mut nums = vec![2,0,2,1,1,0];
    sort_colors(&mut nums);
    println!("{:?}", nums);
}
