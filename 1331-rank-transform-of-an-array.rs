pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
    let mut sorted_arr = arr.clone();
    sorted_arr.sort_unstable();
    sorted_arr.dedup();
    use std::collections::HashMap;
    let mut mmp = HashMap::new();
    for (i, e) in sorted_arr.iter().enumerate() {
        mmp.insert(e, i as i32 + 1);
    }
    arr.iter().map(|e|mmp[e]).collect()
}

fn main() {
    let arr = vec![40,10,20,30];
    println!("{:?}", array_rank_transform(arr));
}
