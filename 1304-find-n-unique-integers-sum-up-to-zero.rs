pub fn sum_zero(n: i32) -> Vec<i32> {
    let mut res: Vec<i32> = (1..n).collect();
    res.push(n * (n-1) / -2);
    res
}

fn main() {
    println!("{:?}", sum_zero(5));
}
