// sort_unstable() + dedup() = "unique()"

pub fn unique_occurrences(arr: Vec<i32>) -> bool {
    use std::collections::HashMap;
    let mut count_map = HashMap::new();
    for num in arr {
        let counter = count_map.entry(num).or_insert(0);
        *counter += 1;
    }
    let mut uniq_vals: Vec<i32> = count_map.values().cloned().collect();
    uniq_vals.sort_unstable(); //必须加这行，不然不是unique。
    uniq_vals.dedup();
    count_map.values().len() == uniq_vals.len()
}

fn main() {
    println!("{:?}", unique_occurrences(vec![26,2,16,16,5,5,26,2,5,20,20,5,2,20,2,2,20,2,16,20,16,17,16,2,16,20,26,16]));
}
