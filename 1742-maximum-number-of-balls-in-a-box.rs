pub fn count_balls(low_limit: i32, high_limit: i32) -> i32 {
    use std::collections::HashMap;
    let mut mmp = HashMap::new();
    for i in low_limit ..= high_limit {
        let a = i.to_string().chars().map(|e|e.to_string().parse::<i32>().unwrap()).sum::<i32>();
        mmp.entry(a).and_modify(|e|*e += 1).or_insert(1);
    }
    mmp.into_values().into_iter().max().unwrap()
}

fn main() {
    println!("{:?}", count_balls(1, 10));
    println!("{:?}", count_balls(5, 15));
    println!("{:?}", count_balls(19, 28));
}
