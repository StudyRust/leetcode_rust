pub fn count_operations(num1: i32, num2: i32) -> i32 {
    if num1 == 0 || num2 == 0 {
        return 0;
    }
    let (mut n1, mut n2, mut res) = (num1, num2, 0);
    loop {
        if n1 == n2 {
            break;
        }
        if n1 >= n2 {
            n1 -= n2;
        } else {
            n2 -= n1;
        }
        res += 1;
    }
    res += 1;
    res
}

fn main() {
    println!("{:?}", count_operations(0, 1));
}
