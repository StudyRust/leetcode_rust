pub fn make_good(s: String) -> String {
    if s.len() == 1 { return s }
    let mut s = s.clone();
    'outer: loop {
        if s.len() == 1 { return s }
        for i in 0..s.len() - 2 {
            if (s.chars().nth(i).unwrap() as i32 - s.chars().nth(i+1).unwrap() as i32).abs() == 32 {
                s.remove(i);
                s.remove(i);
                continue 'outer;
            }
        }
        break;
    }
    if (s.chars().nth(s.len()-1).unwrap() as i32 - s.chars().nth(s.len()-2).unwrap() as i32).abs() == 32 {
        s.remove(s.len()-1);
        s.remove(s.len()-1);
    }
    s
}

fn main() {
    println!("{:?}", make_good("leEeetcode".to_string()));
    println!("{:?}", make_good("abBAcC".to_string()));
    println!("{:?}", make_good("A".to_string()));
    println!("{:?}", make_good("Aa".to_string()));
    println!("{:?}", make_good("asLwGWlSAQq".to_string()));
    println!("{:?}", make_good("XxDFOBKRrkbofdXxeEijJIcCsBikPgfxXFGpKIbSemGivrqqQQRVIgMEPBWpPwbpSCWqWQqwQwcsQqasLwGWlSAQq".to_string()));
    // println!("{:?}", ('b' as i32 -'B' as i32).abs()); // 32
}
