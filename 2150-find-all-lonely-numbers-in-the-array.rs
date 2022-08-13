pub fn find_lonely(nums: Vec<i32>) -> Vec<i32> {
    use std::collections::HashMap;
    let mut mmp = HashMap::new();
    for n in nums {
        let count = mmp.entry(n).or_insert(0);
        *count += 1;
    }
    let mut ret = vec!();
    for (k, v) in &mmp {
        if *v != 1 { continue }
        if mmp.get(&(k-1)) != None { continue }
        if mmp.get(&(k+1)) != None { continue }
        ret.push(*k);
    }
    ret
}

fn main() {
    let nums = vec![10,6,5,8];
    println!("{:?}", find_lonely(nums));
}
