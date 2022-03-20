pub fn num_of_pairs(nums: Vec<String>, target: String) -> i32 {
    let mut res = 0;
    for (i, numi) in nums.iter().enumerate() {
        for (j, numj) in nums.iter().enumerate() {
            if i == j {
                continue;
            }
            let mut tmp = String::new();
            tmp.push_str(numi);
            tmp.push_str(numj);
            if tmp == target {
                res += 1;
            }
        }
    }
    res
}

fn main() {
    let nums = vec!["1","1","1"]
        .iter().map(|e|e.to_string()).collect();
    let target = "11".to_string();
    println!("{:?}", num_of_pairs(nums, target));
}
