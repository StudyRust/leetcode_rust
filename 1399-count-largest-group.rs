pub fn count_largest_group(n: i32) -> i32 {
    let mut count = vec![0;99];
    for i in 1..=n {
        let a = i.to_string().chars().map(|c| c.to_digit(10).unwrap()).sum::<u32>();
        count[a as usize] += 1;
    }
    let m = *count.iter().max().unwrap();
    let mut ret = 0;
    for i in count {
        if i == m {
            ret += 1;
        }
    }
    ret
}

fn main() {
    println!("{:?}", count_largest_group(13));
}
