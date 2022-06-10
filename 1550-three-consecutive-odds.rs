pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
    pub fn is_odd(num: i32) -> bool {
        num % 2 != 0
    }
    let mut count = 0;
    for i in 0..arr.len() {
        if is_odd(arr[i]) {
            count += 1;
        } else {
            count = 0;
        }
        if count == 3 {
            return true;
        }
    }
    false
}

fn main() {
    let arr = vec![1,1,1];
    println!("{:?}", three_consecutive_odds(arr));
}
