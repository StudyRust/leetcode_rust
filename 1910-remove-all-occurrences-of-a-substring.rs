// Rust str API: split_once

pub fn remove_occurrences(s: String, part: String) -> String {
    let mut ret = s;
    loop {
        let parts = ret.split_once(&part);
        if parts == None {
            break
        } else {
            let uwparts = parts.unwrap();
            ret = vec![uwparts.0, uwparts.1].join("");
        }
    }
    ret
}

fn main() {
    let s = "aabababa".to_string();
    let part = "aba".to_string();
    println!("{:?}", remove_occurrences(s, part));
}
