pub fn reformat_number(number: String) -> String {
    let tmp = number.replace("-", "").replace(" ", "");
    let mut tmp_arr: Vec<_> = vec!();
    for i in (0..tmp.len()).step_by(3) {
        let mut tail = i+3;
        if tail > tmp.len() {
            tail = tmp.len();
        }
        tmp_arr.push(&tmp[i..tail]);
    }
    let mut tmp_s = String::new();
    if tmp_arr.len() >= 2 && tmp_arr[tmp_arr.len()-1].len() == 1 {
        tmp_s.push_str(tmp_arr[tmp_arr.len()-2]);
        tmp_s.push_str(tmp_arr[tmp_arr.len()-1]);
        tmp_arr.pop();
        tmp_arr.pop();
        tmp_arr.push(&tmp_s[0..2]);
        tmp_arr.push(&tmp_s[2..4]);
    }
    tmp_arr.join("-")
}

fn main() {
    let number = "9964-".to_string();
    println!("{:?}", reformat_number(number));
}
