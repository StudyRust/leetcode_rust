pub fn generate_the_string(n: i32) -> String {
    let mut res: String;
    if n % 2 == 0 {
        res = "a".repeat(n as usize-1);
        res.push('b');
    } else {
        res = "a".repeat(n as usize);
    }
    res
}

fn main() {
    println!("{:?}", generate_the_string(4));
}
