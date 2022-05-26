pub fn largest_odd_number(num: String) -> String {
    for (i, c) in num.chars().rev().enumerate() {
        if c.to_string().parse::<usize>().unwrap() % 2 != 0 {
            return num[..(num.len()-i)].to_string();
        }
    }
    String::new()
}

fn main() {
    let num = "4206".to_string();
    println!("{:?}", largest_odd_number(num));
}
