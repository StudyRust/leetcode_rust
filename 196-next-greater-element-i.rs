struct Solution;

impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        pub fn next_greater(num: i32, nums2: &[i32]) -> i32 {
            let mut flag = false;
            for i in nums2 {
                if i == &num {
                    flag = true;
                }
                if flag == true && i > &num {
                    return *i;
                }
            }
            return -1;
        }
        nums1.iter().map(|num|next_greater(*num, &nums2)).collect::<Vec<i32>>()
    }
}

fn main() {
    let nums1 = vec![4, 1, 2];
    let nums2 = vec![1, 3, 4, 2];
    println!("{:?}", Solution::next_greater_element(nums1, nums2));
}
