pub fn min_operations(s: String) -> i32 {
    let s_len = s.len();
    let mut final_0 = vec!['0'; s_len];
    let mut final_1 = vec!['1'; s_len];
    let idx: usize;
    if s_len % 2 == 0 { idx = s_len / 2 } else { idx = s_len / 2 + 1 }
    for i in 0..idx{
        final_0[i * 2] = '1';
        final_1[i * 2] = '0';
    }
    let (mut final_0_count, mut final_1_count) = (0, 0);
    for (i, c) in s.chars().collect::<Vec<char>>().iter().enumerate() {
        if final_0[i] != *c { final_0_count += 1 }
        if final_1[i] != *c { final_1_count += 1 }
    }
    *vec![final_1_count, final_0_count].iter().min().unwrap()
}

fn main() {
    let s = "1111".to_string();
    println!("{:?}", min_operations(s));
}
