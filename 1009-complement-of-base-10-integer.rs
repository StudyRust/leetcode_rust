pub fn bitwise_complement(n: i32) -> i32 {
    let a = format!("{:b}", n).chars()
        .map(|e|
            if e == '0' {
                "1"
            } else {
                "0"
            }
        )
        .collect::<Vec<_>>()
        .join("");
    isize::from_str_radix(&a, 2).unwrap() as i32
}

fn main() {
    println!("{:?}", bitwise_complement(7));
}
