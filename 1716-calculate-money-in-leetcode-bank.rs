// 数学总结归纳法
pub fn total_money(n: i32) -> i32 {
    let week = n / 7;
    let day = n % 7;
    7*(1+week)*week/2+21*week+(2*(week+1)+day-1)*day/2
}

fn main() {
    let n = 20;
    println!("{:?}", total_money(n));
}
