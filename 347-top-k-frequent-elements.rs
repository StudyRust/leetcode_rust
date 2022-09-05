pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    use std::collections::HashMap;
    let mut mmp = HashMap::new();
    for num in nums {
        let count = mmp.entry(num).or_insert(0);
        *count += 1;
    }
    let mut mv = mmp.iter().collect::<Vec<_>>();
    mv.sort_by(|a, b|b.1.cmp(&a.1));
    mv[0..k as usize].into_iter().map(|e|*e.0).collect()
}

fn main() {
    let nums = vec![1, 1, 1, 2, 2, 3];
    let k = 2;
    println!("{:?}", top_k_frequent(nums, k));
    println!("{:?}", top_k_frequent(vec![4,1,-1,2,-1,2,3], 2));
}
