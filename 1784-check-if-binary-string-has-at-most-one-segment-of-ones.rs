pub fn check_ones_segment(s: String) -> bool {
    if s == "1".to_string() { return true }
    let cs = s.chars().collect::<Vec<char>>();
    let mut prev_char = '1';
    let mut change_times = 0;
    for c in &cs[1..] {
        if *c != prev_char { change_times += 1 }
        prev_char = *c;
    }
    change_times <= 1
}

fn main() {
    println!("{:?}", check_ones_segment("1001".to_string())); // false
    println!("{:?}", check_ones_segment("110".to_string())); // true
    println!("{:?}", check_ones_segment("1".to_string())); // true
    println!("{:?}", check_ones_segment("10".to_string())); // true
}
