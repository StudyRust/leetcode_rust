pub fn reduction_operations(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let mut mmp = HashMap::new();
    for num in nums {
        mmp.entry(num).and_modify(|e|*e += 1).or_insert(1);
    }
    let mut mmp_vec: Vec<_> = mmp.iter().collect();
    mmp_vec.sort_by(|a, b|a.0.cmp(&b.0));
    // println!("{:?}", mmp_vec);
    let mut ret = 0;
    for (i, r) in mmp_vec.iter().enumerate() {
        ret += i * r.1
    }
    ret as i32
}

fn main() {
    println!("{:?}", reduction_operations(vec![5,1,3]));
    println!("{:?}", reduction_operations(vec![1,1,1]));
    println!("{:?}", reduction_operations(vec![1,1,2,2,3]));
}
