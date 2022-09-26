pub fn partition_array(nums: Vec<i32>, k: i32) -> i32 {
    let mut sorted_nums = nums.clone();
    sorted_nums.sort();
    let mut ret = 1;
    let mut tmp = vec![sorted_nums[0]];
    for num in &sorted_nums[1..] {
        if num - tmp[0] > k {
            tmp = vec![*num];
            ret += 1;
        } else {
            tmp.push(*num);
        }
    }
    ret
}

fn main() {
    println!("{:?}", partition_array(vec![3, 6, 1, 2, 5], 2));
    println!("{:?}", partition_array(vec![1, 2, 3], 1));
    println!("{:?}", partition_array(vec![2, 2, 4, 5], 0));
}
