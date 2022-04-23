pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums1_uniq = nums1.clone();
    let mut nums2_uniq = nums2.clone();
    nums1_uniq.sort_unstable();
    nums1_uniq.dedup();
    nums2_uniq.sort_unstable();
    nums2_uniq.dedup();
    let mut ret = vec!();
    nums2_uniq.retain(|x|!nums1.contains(x));
    nums1_uniq.retain(|x|!nums2.contains(x));
    ret.push(nums1_uniq);
    ret.push(nums2_uniq);
    ret
}

fn main() {
    let nums1 = vec![1,2,3];
    let nums2 = vec![2,4,6];
    println!("{:?}", find_difference(nums1, nums2));
}
