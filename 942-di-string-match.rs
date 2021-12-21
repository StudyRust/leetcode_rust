pub fn di_string_match(s: String) -> Vec<i32> {
    let s_len = s.len();
    let mut arr: Vec<i32> = (0..s_len+1).map(|e|e as i32).collect();
    let mut res: Vec<i32> = vec!();
    for c in s.chars() {
        let op_index;
        if c == 'I' {
            op_index = 0;
        } else {
            op_index = arr.len()-1;
        }
        res.push(arr[op_index]);
        arr.remove(op_index);
    }
    res.push(*arr.last().unwrap());
    res
}

fn main() {
    println!("{:?}", di_string_match("IDID".to_string()));
}
