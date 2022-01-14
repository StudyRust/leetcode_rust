pub fn find_lucky(arr: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let mut mmp = HashMap::new();
    for num in arr {
        let counter = mmp.entry(num).or_insert(0);
        *counter += 1;
    }
    mmp.retain(|k, v|k==v);
    let mut mmp_vec: Vec<_> = mmp.iter().collect();
    mmp_vec.sort_by(|a, b|b.0.cmp(a.0));
    if mmp_vec.iter().count() == 0 {
        -1
    } else {
        *mmp_vec[0].0
    }
}

fn main() {
    let arr = vec![2,2,3,4];
    println!("{:?}", find_lucky(arr));
}
