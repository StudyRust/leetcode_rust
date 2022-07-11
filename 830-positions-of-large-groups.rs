pub fn large_group_positions(s: String) -> Vec<Vec<i32>> {
    let chars = s.chars().collect::<Vec<char>>();
    let mut prev_char = chars[0];
    let mut count = 1;
    let mut arr = vec!();
    for c in &chars[1..] {
        if *c == prev_char {
            count += 1;
        } else {
            arr.push(count);
            count = 1;
        }
        prev_char = *c;
    }
    arr.push(count);
    let mut ret = vec!();
    let mut sum = 0;
    for cc in arr {
        if cc > 2 {
            ret.push(vec![sum, sum + cc - 1]);
        }
        sum += cc;
    }
    ret
}

fn main() {
    let s = "abcdddeeeeaabbbcd".to_string();
    println!("{:?}", large_group_positions(s));
}
