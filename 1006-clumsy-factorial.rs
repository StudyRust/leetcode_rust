pub fn clumsy(n: i32) -> i32 {
    if n == 1 { return 1 }
    if n == 2 { return 2 }
    let mut ret = n * (n-1) / (n-2) + n-3;
    let mut tmp = 0;
    for (i, num) in (1..=n-4).rev().enumerate() {
        match i % 4 {
            0 => { tmp = num },
            1 => { tmp *= num },
            2 => {
                ret -= (tmp as f64 / num as f64).floor() as i32;
                tmp = 0
            },
            3 => { ret += num },
            _ => { },
        }
    }
    ret - tmp
}

fn main() {
    println!("{:?}", clumsy(2));
}
