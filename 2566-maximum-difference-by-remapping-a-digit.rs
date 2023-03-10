pub fn min_max_difference(num: i32) -> i32 {
    let num_str = num.to_string().chars().collect::<Vec<char>>();
    // println!("{:?}", num_str[0]);
    let mut idx = 0;
    for (i, c) in num_str.iter().enumerate() {
        if *c != '9' {
            idx = i;
            break
        }
    }
    let mut max_str = String::new();
    let mut min_str = String::new();
    for c in &num_str {
        if *c == num_str[idx] {
            max_str.push('9');
        } else {
            max_str.push(*c);
        }
        if *c == num_str[0] {
            min_str.push('0');
        } else {
            min_str.push(*c);
        }
    }
    
    let max = max_str.parse::<i32>().unwrap();
    let min = min_str.parse::<i32>().unwrap();
    max - min
}

fn main() {
    println!("{:?}", min_max_difference(11891)); // 99009
    println!("{:?}", min_max_difference(90));    // 99
    println!("{:?}", min_max_difference(2));     // 9
}
