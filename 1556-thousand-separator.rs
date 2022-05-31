pub fn thousand_separator(n: i32) -> String {
    let s = n.to_string();
    let mut v: Vec<_> = vec!();
    for i in 0..s.len()/3 {
        v.insert(0, &s[s.len()-i*3-3..s.len()-i*3]);
    }
    let mo = s.len()%3;
    if mo != 0 {
        v.insert(0, &s[..mo]);
    }
    v.join(".")
}

fn main() {
    let n = 1234;
    println!("{:?}", thousand_separator(n));
}
