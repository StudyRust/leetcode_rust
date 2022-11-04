pub fn min_set_size(arr: Vec<i32>) -> i32 {
    let arr_len = arr.len();
    use std::collections::HashMap;
    let mut mmp = HashMap::new();
    for e in arr {
        mmp.entry(e).and_modify(|e|*e+=1).or_insert(1);
    }
    let mut mmp_vec: Vec<_> = mmp.iter().collect();
    mmp_vec.sort_by(|b, a|a.1.cmp(&b.1));
    let mut tmp = 0;
    let mut ret = 0;
    for p in mmp_vec {
        tmp += p.1;
        ret += 1;
        if tmp >= arr_len / 2 { break }
    }
    ret
}

fn main() {
    println!("{:?}", min_set_size(vec![3, 3, 3, 3, 5, 5, 5, 2, 2, 7]));
    println!("{:?}", min_set_size(vec![7, 7, 7, 7, 7, 7]));
    println!("{:?}", min_set_size(vec![1, 9]));
}
