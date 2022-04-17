pub fn longest_nice_substring(s: String) -> String {
    fn is_nice(s: &str) -> bool {
        let mut a = s.to_string().chars().collect::<Vec<char>>();
        a.sort_unstable();
        a.dedup();
        println!("{:?}", a);
        true
    }
    for i in 0..s.len()-1 {
        for j in i+1..s.len() {
            let a = &s[i..j];
            if is_nice(a) {
                // println!("{:?}", a);
                // return a.to_strin();
            }
        }
    }
    String::new()
}

fn main() {
    let s = "YazaAay".to_string();
    println!("{:?}", longest_nice_substring(s));
}
