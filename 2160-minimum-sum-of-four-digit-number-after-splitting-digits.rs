pub fn minimum_sum(num: i32) -> i32 {
    let mut sorted_num_chars: Vec<char> = num.to_string().chars().collect();
    sorted_num_chars.sort();
    let mut new1 = String::new();
    let mut new2 = String::new();
    println!("{:?}", sorted_num_chars);
    for (i, e) in sorted_num_chars.iter().enumerate() {
        if i % 2 == 0 {
            new1.push(*e);
        } else {
            new2.push(*e);
        }
    }
    let new1_num: i32 = new1.parse().unwrap();
    let new2_num: i32 = new2.parse().unwrap();
    new1_num + new2_num
}

fn main() {
    let num = 2932;
    println!("{:?}", minimum_sum(num));
}
