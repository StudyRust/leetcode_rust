pub fn calculate_tax(brackets: Vec<Vec<i32>>, income: i32) -> f64 {
    let mut ret = 0.0_f64;
    let mut prev = 0.0_f64;
    let mut left_income: f64 = income.into();
    for b in brackets {
        if left_income <= 0.0 { break }
        if left_income > b[0] as f64 - prev {
            ret += ( b[0] as f64 - prev ) * b[1] as f64/ 100.0_f64;
            left_income -= b[0] as f64 - prev;
            prev = b[0] as f64;
        } else {
            ret += left_income * b[1] as f64 / 100.0_f64;
            break;
        }
    }
    ret
}

fn main() {
    let brackets = vec![vec![3,50], vec![7,10], vec![12,25]];
    let income = 10;
    println!("{:?}", calculate_tax(brackets, income));
}
