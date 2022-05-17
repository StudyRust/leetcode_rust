pub fn two_out_of_three(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>) -> Vec<i32> {
    use std::collections::HashSet;
    let mut ret = HashSet::new();
    for i in &nums1 {
        for j in &nums2 {
            if i == j {
                ret.insert(*i);
            }
        }
        for k in &nums3 {
            if i == k {
                ret.insert(*i);
            }
        }
    }
    for i in &nums2 {
        for j in &nums3 {
            if i == j {
                ret.insert(*i);
            }
        }
    }
    ret.into_iter().collect::<Vec<i32>>()
}

fn main() {
    let nums1 = vec![1,1,3,2];
    let nums2 = vec![2,3];
    let nums3 = vec![3];
    println!("{:?}", two_out_of_three(nums1, nums2, nums3));
}
