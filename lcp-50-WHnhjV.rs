pub fn give_gem(gem: Vec<i32>, operations: Vec<Vec<i32>>) -> i32 {
    let mut gem = gem.clone();
    for op in operations {
        let minus = (gem[op[0] as usize] as f64 / 2.0_f64).floor() as i32;
        gem[op[0] as usize] -= minus;
        gem[op[1] as usize] += minus;
    }
    *gem.iter().max().unwrap() - *gem.iter().min().unwrap()
}

fn main() {
    let gem = vec![3,1,2];
    let operations = vec![vec![0,2],vec![2,1],vec![2,0]];
    println!("{:?}", give_gem(gem, operations));
}
