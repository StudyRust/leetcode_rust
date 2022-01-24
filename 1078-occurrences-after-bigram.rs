pub fn find_ocurrences(text: String, first: String, second: String) -> Vec<String> {
    let strs: Vec<String> = text.split(" ").map(|s|s.to_string()).collect();
    let tail_index = &strs.len() - 2;
    let mut res: Vec<String> = Vec::new();
    for i in 0..tail_index {
        if strs[i] == first && strs[i+1] == second {
            res.push(strs[i+2].clone());
        }
    }
    res
}

fn main() {
    let text = "we will we will rock you".to_string();
    let first = "we".to_string();
    let second = "will".to_string();

    println!("{:?}", find_ocurrences(text, first, second));
}
