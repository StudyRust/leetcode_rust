struct Solution {

}

impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        if dividend == -2147483648 && divisor == -1 {
            2147483647 
        } else {
            dividend / divisor
        }
    }
}

fn main() {
    println!("{:?}", Solution::divide(10, 3));
}
