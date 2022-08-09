pub fn get_kth(lo: i32, hi: i32, k: i32) -> i32 {
    use std::collections::HashMap;
    let mut mmp = HashMap::new();
    for i in lo..=hi {
        let mut tmp = i;
        let mut count = 0;
        while tmp != 1 {
            if tmp % 2 == 0 {
                tmp /= 2;
            } else {
                tmp = 3 * tmp + 1;
            }
            count += 1;
        }
        mmp.entry(count).or_insert(Vec::new()).push(i);
    }
    let mut mmp_vec: Vec<_> = mmp.iter().collect();
    mmp_vec.sort_by(|a, b|a.0.cmp(b.0));
    *mmp_vec.into_iter().map(|e|e.1).collect::<Vec<_>>().into_iter().flatten().collect::<Vec<_>>()[k as usize - 1]
}

fn main() {
    println!("{:?}", get_kth(12, 15, 2));
}
