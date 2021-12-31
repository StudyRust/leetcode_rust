pub fn minimum_abs_difference(arr: Vec<i32>) -> Vec<Vec<i32>> {
    let mut sorted_arr = arr.clone();
    sorted_arr.sort();
    use std::collections::HashMap;
    let mut mmp = HashMap::new();
    for i in 1..arr.len() {
        mmp.entry((sorted_arr[i]-sorted_arr[i-1]).abs())
            .or_insert(Vec::new())
            .push(vec!(sorted_arr[i-1], sorted_arr[i]));
    }
    let mut mmp_vec: Vec<_> = mmp.iter().collect();
    mmp_vec.sort_by(|a, b|a.0.cmp(b.0));
    mmp_vec[0].1.to_vec()
}

fn main() {
    let arr = vec![3, 8, -10, 23, 19, -4, -14, 27];
    println!("{:?}", minimum_abs_difference(arr));
}
