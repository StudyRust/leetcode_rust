pub fn number_of_pairs(nums: Vec<i32>) -> Vec<i32> {
    use std::collections::HashMap;
    let mut mmp = HashMap::new();
    let nums_len = nums.len();
    for n in nums {
        mmp.entry(n).and_modify(|e|*e += 1).or_insert(1);
    }
    let mut a = 0;
    for (_, v) in mmp {
        a += v % 2;
    }
    vec!((nums_len as i32 - a) / 2, a)
}

fn main() {
    println!("{:?}", number_of_pairs(vec![1,3,2,1,3,2,2]));
    println!("{:?}", number_of_pairs(vec![1,1]));
    println!("{:?}", number_of_pairs(vec![0]));
}
