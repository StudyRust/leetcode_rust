pub fn cells_in_range(s: String) -> Vec<String> {
    let a: Vec<_> = s.split(":").collect();
    let start_alpha = a[0].chars().nth(0).unwrap();
    let start_num = a[0].chars().nth(1).unwrap();
    let end_alpha = a[1].chars().nth(0).unwrap();
    let end_num = a[1].chars().nth(1).unwrap();
    let mut ret: Vec<String> = vec!();
    for a in start_alpha..=end_alpha {
        for n in start_num..=end_num {
            let mut tmp = String::new();
            tmp.push(a);
            tmp.push(n);
            ret.push(tmp);
        }
    }
    ret
}

fn main() {
    let s = "A1:F1".to_string();
    println!("{:?}", cells_in_range(s));
}
