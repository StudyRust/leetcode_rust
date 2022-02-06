pub fn binary_gap(n: i32) -> i32 {
    let bin_chars: Vec<char> = format!("{:b}", n).chars().collect();
    let mut one_arr = vec!();
    for (i, c) in bin_chars.iter().enumerate() {
        if c == &'1' {
            one_arr.push(i);
        }
    }
    if &one_arr.len() < &2 {
        0
    } else {
        let mut minus_arr = vec!();
        for i in 1..one_arr.len() {
            minus_arr.push(one_arr[i] - one_arr[i-1]);
        }
        *minus_arr.iter().max().unwrap() as i32
    }
}

fn main() {
    println!("{:?}", binary_gap(22));
}
