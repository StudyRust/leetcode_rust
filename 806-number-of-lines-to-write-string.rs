pub fn number_of_lines(widths: Vec<i32>, s: String) -> Vec<i32> {
    use std::collections::HashMap;
    let mut mmp = HashMap::new();
    let cs = vec!['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'];
    for (i, c) in cs.iter().enumerate() {
        mmp.insert(c, widths[i]);
    }
    let mut current = 0;
    let mut line = 1;
    for c in s.chars().collect::<Vec<char>>() {
        if current + mmp[&c] > 100 {
            line += 1;
            current = mmp[&c];
        } else {
            current += mmp[&c];
        }
    }
    vec![line, current]
}

fn main() {
    let widths = vec![10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10];
    let s = "abcdefghijklmnopqrstuvwxyz".to_string();
    println!("{:?}", number_of_lines(widths, s));
    let widths = vec![4,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10];
    let s = "bbbcccdddaaa".to_string();
    println!("{:?}", number_of_lines(widths, s));
}
