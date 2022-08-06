pub fn check_if_can_break(s1: String, s2: String) -> bool {
    let mut sorted_s1_chars = s1.chars().collect::<Vec<char>>();
    let mut sorted_s2_chars = s2.chars().collect::<Vec<char>>();
    sorted_s1_chars.sort();
    sorted_s2_chars.sort();
    let mut s1_gr_s2 = true;
    for i in 0..sorted_s1_chars.len() {
        if sorted_s1_chars[i] > sorted_s2_chars[i] {
            break;
        }
        if sorted_s1_chars[i] < sorted_s2_chars[i] {
            s1_gr_s2 = false;
            break;
        }
    }
    for i in 0..sorted_s1_chars.len() {
        if sorted_s1_chars[i] > sorted_s2_chars[i] && !s1_gr_s2 {
            return false;
        }
        if sorted_s1_chars[i] < sorted_s2_chars[i] && s1_gr_s2 {
            return false;
        }
    }
    true
}

fn main() {
    let s1 = "bxfowqvnrhuzwqohquamvszkvunb".to_string();
    let s2 = "xjegbjccjjxfnsiearbsgsofywtq".to_string();
    println!("{:?}", check_if_can_break(s1, s2));
}

// "bxfowqvnrhuzwqohquamvszkvunb"
// "xjegbjccjjxfnsiearbsgsofywtq"
