pub fn rotate_string(s: String, goal: String) -> bool {
    if s == goal {
        return true;
    }
    let mut s_chars: Vec<char> = s.chars().collect();
    let goal_chars: Vec<char> = goal.chars().collect();
    for _i in 0..s.len() {
        s_chars.rotate_left(1);
        if s_chars == goal_chars {
            return true;
        }
    }
    return false;
}

fn main() {
    let s = "abcde".to_string();
    let goal = "cdaeb".to_string();
    println!("{:?}", rotate_string(s, goal));
}
