pub fn add_digits(num: i32) -> i32 {
    let mut sum = num;
    while sum >= 10 {
        let arr: Vec<char> = sum.to_string().chars().collect();
        sum = arr.iter().map(|e|*e as i32 - '0' as i32).sum();
    }
    sum
}

fn main() {
    println!("{:?}", add_digits(38));
}
