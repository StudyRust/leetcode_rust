pub fn rotated_digits(n: i32) -> i32 {
    pub fn is_good(e: i32) -> bool {
        let digit_chars = e.to_string().chars().collect::<Vec<char>>();
        let mut count = 0;
        for digit_char in digit_chars {
            if digit_char == '3' || digit_char == '4'
            || digit_char == '7' {
                return false
            }
            if digit_char == '2' || digit_char == '5'
            || digit_char == '6' || digit_char == '9' {
                count += 1
            }
        }
        count > 0
    }
    let mut nums = (1..=n).collect::<Vec<i32>>();
    nums.retain(|e|is_good(*e));
    nums.len() as i32
}

fn main() {
    println!("{:?}", rotated_digits(10));
}
