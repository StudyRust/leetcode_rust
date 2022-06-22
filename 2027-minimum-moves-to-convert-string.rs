pub fn minimum_moves(s: String) -> i32 {
    let cs = s.chars().collect::<Vec<char>>();
    let mut i = 0;
    let mut ret = 0;
    while i < s.len() {
        if cs[i] == 'X' {
            i += 3;
            ret += 1;
        } else {
            i += 1;
        }
    }
    ret
}

fn main() {
    let s = "XXOX".to_string();
    println!("{:?}", minimum_moves(s));
}
