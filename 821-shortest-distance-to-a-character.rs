pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
    let mut ch_indices = vec!();
    for (i, ch) in s.chars().enumerate() {
        if c == ch {
            ch_indices.push(i as i32);
        }
    }
    let mut ret: Vec<i32> = vec!();
    for (i, ch) in s.chars().enumerate() {
        if c == ch {
            ret.push(0);
        } else {
            ret.push(ch_indices.iter().map(|e|(e-i as i32).abs()).min().unwrap());
        }
    }
    ret
}

fn main() {
    let s = "loveleetcode".to_string();
    let c = 'e';
    println!("{:?}", shortest_to_char(s, c));
}
