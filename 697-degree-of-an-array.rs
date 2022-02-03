pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
    let mut res = Vec::new();
    use std::collections::HashMap;
    let mut counter_map = HashMap::new();
    for num in &nums {
        let count = counter_map.entry(num).or_insert(0);
        *count += 1;
    }
    let mut counter_vec: Vec<_> = counter_map.iter().collect();
    counter_vec.sort_by(|b, a|a.1.cmp(b.1));
    let du = counter_vec[0].1;
    counter_vec.retain(|e|e.1 == du);
    for pair in counter_vec {
        let key_element = pair.0;
        let index_0 = nums.iter().position(|&r|r == **key_element).unwrap();
        let index_1 = nums.iter().rev().position(|&r|r == **key_element).unwrap();
        let tmp = (&nums.len() - index_1 - index_0) as i32;
        res.push(tmp);
    }
    *res.iter().min().unwrap()
}

fn main() {
    let nums = vec![1,2,2,3,1];
    println!("{:?}", find_shortest_sub_array(nums));
}
