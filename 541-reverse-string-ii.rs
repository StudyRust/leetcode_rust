// good，练习chunk的使用。
pub fn reverse_str(s: String, k: i32) -> String {
    let mut ret_c = s.chars().collect::<Vec<char>>();
    let mut idx = 0;
    for chunk in ret_c.chunks_mut(k as usize) {
        if idx % 2 == 0 {
            let mut tmp = chunk.to_vec();
            tmp.reverse();
            for (i, elem) in chunk.iter_mut().enumerate() {
                *elem = tmp[i];
            }
        }
        idx += 1;
    }
    ret_c.iter().collect()
}

fn main() {
    let s = "abcd".to_string();
    let k = 2;
    println!("{:?}", reverse_str(s, k));
}
