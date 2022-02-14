pub fn is_same_after_reversals(num: i32) -> bool {
    if num == 0 {
        return true;
    }
    num % 10 != 0
}

fn main() {
    println!("{:?}", is_same_after_reversals());
}
