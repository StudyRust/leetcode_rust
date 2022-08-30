pub fn minimum_cost(cost: Vec<i32>) -> i32 {
    let mut sorted = cost.clone();
    sorted.sort_by(|a, b|b.cmp(&a));
    let mut ret = 0;
    for (i, e) in sorted.iter().enumerate() {
        if i % 3 == 2 { continue }
        ret += e;
    }
    ret
}

fn main() {
    let cost = vec![6, 5, 7, 9, 2, 2];
    println!("{:?}", minimum_cost(cost));
}
