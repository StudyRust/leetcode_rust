pub fn frequency_sort(nums: Vec<i32>) -> Vec<i32> {
    use std::collections::HashMap;
    let mut mmp: HashMap<i32, Vec<i32>> = HashMap::new();
    for i in nums {
        mmp.entry(i).or_insert(Vec::new()).push(i);
    }
    let mut mmp_vec: Vec<_> = mmp.iter().collect();
    mmp_vec.sort_by(|a, b| b.0.cmp(&a.0));
    mmp_vec.sort_by(|a, b| a.1.len().cmp(&b.1.len()));
    let mut ret: Vec<i32> = vec!();
    for e in mmp_vec {
        ret.extend(e.1);
    }
    ret
}

fn main() {
    let nums = vec![-1,1,-6,4,5,-6,1,4,1];
    println!("{:?}", frequency_sort(nums));
}
