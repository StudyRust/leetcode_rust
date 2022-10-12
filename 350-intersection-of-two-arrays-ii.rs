pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    use std::collections::HashMap;
    let mut mmp1 = HashMap::new();
    let mut mmp2 = HashMap::new();
    let mut ret = vec![];
    for num in nums1 {
        mmp1.entry(num).and_modify(|m|*m += 1).or_insert(1);
    }
    for num in nums2 {
        mmp2.entry(num).and_modify(|m|*m += 1).or_insert(1);
    }
    // println!("{:?} {:?}", mmp1, mmp2);
    if mmp1.len() < mmp2.len() {
        for (k1, v1) in mmp1 {
            let v2 = mmp2.get(&k1);
            if v2.is_some() {
                for _ in 0..**[v2.unwrap(), &v1].iter().min().unwrap() {
                    ret.push(k1);
                }
            }
        }
    } else {
        for (k2, v2) in mmp2 {
            let v1 = mmp1.get(&k2);
            if v1.is_some() {
                for _ in 0..**[v1.unwrap(), &v2].iter().min().unwrap() {
                    ret.push(k2);
                }
            }
        }
    }
    ret
}

fn main() {
    println!("{:?}", intersect(vec![1,2,2,1], vec![2,2]));
    println!("{:?}", intersect(vec![4,9,5], vec![9,4,9,8,4]));
}
