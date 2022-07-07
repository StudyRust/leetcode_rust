// to_lowercase() 和 to_ascii_uppercase()的区别，一个是针对字符串，一个是针对字符。
pub fn capitalize_title(title: String) -> String {
    let mut ret_arr: Vec<String> = vec![];
    for word in title.split_ascii_whitespace() {
        if word.len() < 3 {
            ret_arr.push(word.to_lowercase());
        } else {
            let mut tmp: Vec<char> = word.to_lowercase().chars().collect();
            tmp[0] = tmp[0].to_ascii_uppercase();
            ret_arr.push(tmp.iter().collect::<String>());
        }
    }
    ret_arr.join(" ")
}

fn main() {
    let title = "capiTalIze OF titLe".to_string();
    println!("{:?}", capitalize_title(title));
}
