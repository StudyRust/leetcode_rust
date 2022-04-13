pub fn min_deletion_size(strs: Vec<String>) -> i32 {
    let index = strs.get(0).unwrap().len();
    let mut ret = 0;
    'outer: for i in 0..index {
        let mut c = ' ';
        for s in &strs {
            let cc = s.chars().nth(i).unwrap();
            if c > cc {
                ret += 1;
                continue 'outer;
            }
            c = cc;
        }
    }
    ret
}

fn main() {
    let strs = vec!["zyx","wvu","tsr"]
        .iter().map(|e|e.to_string()).collect();
    println!("{:?}", min_deletion_size(strs));
}
