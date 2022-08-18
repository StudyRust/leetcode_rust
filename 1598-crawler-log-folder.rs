pub fn min_operations(logs: Vec<String>) -> i32 {
    let mut ret = 0;
    for log in logs {
        if log.contains("..") {
            if ret > 0 { ret -= 1 }
        } else if log.contains(".") {
            continue
        } else {
            ret += 1
        }
    }
    ret
}

fn main() {
    let logs: Vec<String> = ["d1/","d2/","../","d21/","./"].iter().map(|e|e.to_string()).collect();
    println!("{:?}", min_operations(logs));
}
