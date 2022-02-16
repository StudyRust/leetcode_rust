pub fn find_special_integer(arr: Vec<i32>) -> i32 {
    let limit_line = arr.len() as f64 * 0.25;
    use std::collections::HashMap;
    let mut mmp = HashMap::new();
    let mut res: i32 = 0;
    for num in arr {
        let counter = mmp.entry(num).or_insert(0.0);
        *counter += 1.0;
        if *counter > limit_line {
            res = num;
            break;
        }
    }
    res
}

fn main() {
    let arr = vec![1,2,2,6,6,6,6,7,10];
    println!("{:?}", find_special_integer(arr));
}
