pub fn digit_sum(s: String, k: i32) -> String {
    let mut ret = s.clone();
    while ret.len() > k as usize {
        let mut tmp = String::new();
        for ch in ret.chars().collect::<Vec<char>>().chunks(k as usize) {
            let mut sum = 0;
            for c in ch {
                sum += c.to_string().parse::<i32>().unwrap();
            }
            tmp.push_str(&sum.to_string());
        }
        ret = tmp;
    }
    ret
}

fn main() {
    println!("{:?}", digit_sum("11111222223".to_string(), 3));
    println!("{:?}", digit_sum("00000000".to_string(), 3));
}
