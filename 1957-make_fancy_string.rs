fn main() {
    let str = "aab".to_string();
    let a = make_fancy_string(str);
    println!("{:?}", a);
}

// 1957 https://leetcode-cn.com/problems/delete-characters-to-make-fancy-string/
// leeetcode    leetcode
// aaabaaaa     aabaa
// aab          aab
pub fn make_fancy_string(s: String) -> String {
    let mut flag_c = '*';
    let mut counter = 1;
    let mut res_str = String::from("");
    for c in s.chars() {
        if c != flag_c {
            flag_c = c;
            counter = 1;
        } else {
            counter += 1;
        }
        if counter > 2 {
            continue;
        } else {
            res_str.push(c);
        }
    }
    res_str
}