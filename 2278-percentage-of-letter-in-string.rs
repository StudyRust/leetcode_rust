pub fn percentage_letter(s: String, letter: char) -> i32 {
    let mut counter = 0;
    for c in s.chars().collect::<Vec<char>>() {
        if c == letter {
            counter += 1
        }
    }
    (100.0 * counter as f64 / s.len() as f64) as i32
}

fn main() {
    println!("{:?}", percentage_letter("foobar".to_string(), 'o'));
}
