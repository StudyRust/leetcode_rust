pub fn longest_nice_substring(s: String) -> String {
    use std::collections::HashMap;
    pub fn is_nice_str(s: &str) -> bool {
        let mut mmp = HashMap::new();
        for c in s.chars() {
            let count = mmp.entry(c).or_insert(0);
            *count += 1;
        }
        let keys: Vec<char> = mmp.into_keys().collect();
        if keys.len() % 2 !=0 {
            false
        } else {
            let mut tmp: Vec<_> = keys.iter().map(|e|e.to_lowercase().to_string()).collect();
            tmp.sort_unstable();
            tmp.dedup();
            keys.len() % tmp.len() == 0 && keys.len() / tmp.len() == 2
        }
    }
    for i in (2..=s.len()).rev() {
        for j in 0..=(s.len()-i) {
            if is_nice_str(&s[j..=j+i-1]) {
                return s[j..=j+i-1].to_string();
            }
        }
    }
    String::new()
}

fn main() {
    let s = "dDzeE".to_string();
    println!("{:?}", longest_nice_substring(s));
}
