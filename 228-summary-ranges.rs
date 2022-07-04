pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
    let mut ret = vec!();
    if nums.len() == 0 {
        return ret;
    }
    pub fn assemble(current: Vec<i32>) -> String {
        if current.len() == 1 {
            current[0].to_string()
        } else {
            let f = current.first().unwrap().to_string();
            let l = current.last().unwrap().to_string();
            vec!(f, "->".to_string(), l).join("")
        }
    }
    let mut prev = nums[0];
    let mut current = vec!(prev);
    for num in &nums[1..] {
        if num - prev != 1 {
            ret.push(assemble(current));
            current = vec!(*num);
        } else {
            current.push(*num);
        }
        prev = *num;
    }
    ret.push(assemble(current));
    ret
}

fn main() {
    let nums = vec![];
    println!("{:?}", summary_ranges(nums));
}
