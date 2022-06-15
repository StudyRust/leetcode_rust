pub fn largest_good_integer(num: String) -> String {
    pub fn is_good(s: &str) -> bool {
        let mut tmp = vec!();
        for c in s.chars() {
            tmp.push(c);
        }
        tmp.sort_unstable();
        tmp.dedup();
        tmp.len() == 1
    }
    let mut sta = -1;
    let mut ret = String::new();
    for i in 0..num.len()-2 {
        let tmp = &num[i..i+3];
        let n = tmp.parse::<i32>().unwrap();
        if is_good(tmp) && n > sta {
            sta = n;
            ret = tmp.to_string();
        }
    }
    ret
}

fn main() {
    let num = "2300019".to_string();
    println!("{:?}", largest_good_integer(num));
}
