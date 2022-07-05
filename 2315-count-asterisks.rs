pub fn count_asterisks(s: String) -> i32 {
    let mut outside = true;
    let mut tmp_s = String::new();
    for c in s.chars() {
        if outside {
            if c != '|' {
                tmp_s.push(c);
            } else {
                outside = false;
            }
        } else {
            if c == '|' {
                outside = true;
            }
        }
    }
    tmp_s.matches(|c|c=='*').count() as i32
}

fn main() {
    println!("{:?}",count_asterisks("yo|uar|e**|b|e***au|tifu|l".to_string()));
}
