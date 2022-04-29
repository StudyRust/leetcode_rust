struct Solution;

impl Solution {
    pub fn kth_distinct(arr: Vec<String>, k: i32) -> String {
    	use std::collections::HashMap;
    	let mut mmp = HashMap::new();
    	for a in &arr {
    		let count = mmp.entry(a).or_insert(0);
    		*count += 1;
    	}
    	let mut tmp = 0;
    	for a in &arr {
    		if mmp[&a] == 1 {
    			tmp += 1;
    		}
    		if tmp == k {
    			return a.to_string();
    		}
    	}
    	String::new()
    }
}

fn main() {
	let arr: Vec<String> = vec!["d","b","c","b","c","a"].iter().map(|e|e.to_string()).collect();
	let k = 2;
	println!("{:?}", Solution::kth_distinct(arr, k));
}
