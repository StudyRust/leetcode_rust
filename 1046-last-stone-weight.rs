pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
    let mut stones = stones.clone();
    while stones.len() > 1 {
        stones.sort_by(|a, b|b.cmp(&a));
        if stones[0] == stones[1] {
            stones.remove(0);
        } else {
            stones[1] = (stones[1]-stones[0]).abs();
        }
        stones.remove(0);
    }
    if stones.len() > 0 {
        stones[0]
    } else {
        0
    }
}

fn main() {
    println!("{:?}", last_stone_weight(vec![2,2]));
}
