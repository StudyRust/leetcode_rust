pub fn get_lucky(s: String, k: i32) -> i32 {
    pub fn luck(s: String) -> String {
        s.chars().map(|e|e.to_string().parse::<i32>().unwrap()).sum::<i32>().to_string()
    }
    let mut ret = s.chars().map(|e|(e as i32 - 'a' as i32 + 1).to_string()).collect::<Vec<String>>().join("");
    for _ in 0..k {
        ret = luck(ret);
    }
    ret.parse::<i32>().unwrap()
}

fn main() {
    println!("{:?}", get_lucky("iiii".to_string(), 1));
}
