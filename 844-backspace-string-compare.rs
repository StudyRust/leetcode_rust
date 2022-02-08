pub fn backspace_compare(s: String, t: String) -> bool {
    fn get_chars(s: String) -> Vec<char> {
        let s_chars: Vec<char> = s.chars().collect();
        let mut cs = vec!();
        for c in s_chars {
            if c != '#' {
                cs.push(c);
            } else {
                cs.pop();
            }
        }
        cs
    }
    get_chars(s) == get_chars(t)
}

fn main() {
    let s = "ab##".to_string();
    let t = "c#d#".to_string();
    println!("{:?}", backspace_compare(s, t));
}
