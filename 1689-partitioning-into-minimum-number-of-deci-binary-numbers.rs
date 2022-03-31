pub fn min_partitions(n: String) -> i32 {
    n.chars().map(|e|e.to_digit(10)).max().unwrap().unwrap() as i32
}

fn main() {
    let n = "27346209830709182346".to_string();
    println!("{:?}", min_partitions(n));
}
