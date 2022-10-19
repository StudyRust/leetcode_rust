pub fn find_different_binary_string(nums: Vec<String>) -> String {
    let n = nums[0].len();
    let mut arr = (0..=2_i32.pow(n as u32)).map(|e|e as i32).collect::<Vec<i32>>();
    for num in nums {
        let intval = isize::from_str_radix(&num, 2).unwrap();
        arr[intval as usize] = -1;
    }
    let mut ret = -1;
    for i in arr {
        if i != -1 {
            ret = i;
            break
        }
    }
    let ret = format!("{:b}", ret);
    let mut prefix = String::new();
    if ret.len() < n {
        for _ in 0 .. (n - ret.len()) {
            prefix.push('0')
        }
    }
    [prefix, ret].concat()
}

fn main() {
    println!("{:?}", find_different_binary_string(vec!["01".to_string(),"10".to_string()]));
    println!("{:?}", find_different_binary_string(vec!["00".to_string(),"01".to_string()]));
    println!("{:?}", find_different_binary_string(vec!["111".to_string(),"011".to_string(),"001".to_string()]));
}
