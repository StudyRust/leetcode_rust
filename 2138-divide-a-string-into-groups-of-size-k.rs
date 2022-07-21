pub fn divide_string(s: String, k: i32, fill: char) -> Vec<String> {
    let mut ret = s.chars()
        .collect::<Vec<char>>().chunks(k as usize)
        .collect::<Vec<_>>().iter().map(|v|v.into_iter().collect::<String>())
        .collect::<Vec<String>>();
    let last_len = ret.last().unwrap().len();
    let l = ret.len();
    if last_len != k as usize {
        for _ in 0..k as usize - last_len {
            ret[l-1].push(fill);
        }
    }
    ret
}

fn main() {
    let s = "abcdefghij".to_string();
    let k = 3;
    let fill = 'x';
    println!("{:?}", divide_string(s, k, fill));
}
