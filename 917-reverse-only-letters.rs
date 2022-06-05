pub fn reverse_only_letters(s: String) -> String {
    let mut idxs: Vec<usize> = vec!();
    let mut tmp = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_alphabetic() {
            idxs.push(i);
            tmp.push(c);
        }
    }
    let mut s = s;
    unsafe {
        let vec = s.as_mut_vec();
        let tmp_vec = tmp.as_mut_vec();
        tmp_vec.reverse();
        for (i, c) in tmp_vec.iter().enumerate() {
            vec[idxs[i]] = *c;
        }
    }
    s
}

fn main() {
    let s = "Test1ng-Leet=code-Q!".to_string();
    println!("{:?}", reverse_only_letters(s));
}
