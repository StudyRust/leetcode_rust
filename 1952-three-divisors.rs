pub fn is_three(n: i32) -> bool {
    if n < 3 {
        return false;
    }
    let mut count = 0;
    for i in 2..n {
        if n % i == 0 {
            count += 1;
        }
        if count > 1 {
            return false;
        }
    }
    count == 1
}

fn main() {
    println!("{:?}", is_three(4));
}
