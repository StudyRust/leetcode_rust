// (2..num).select{|e|e.to_s.chars.map(&:to_i).sum % 2 == 0}.count

pub fn count_even(num: i32) -> i32 {
    let mut ret = 0;
    for i in 2..=num {
        if i.to_string().chars().map(|e|e.to_string().parse::<i32>().unwrap()).sum::<i32>() % 2 == 0 {
            ret += 1;
        }
    }
    ret
}

fn main() {
    println!("{:?}", count_even(30));
}
