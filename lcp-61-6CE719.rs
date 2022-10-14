pub fn temperature_trend(temperature_a: Vec<i32>, temperature_b: Vec<i32>) -> i32 {
    let (mut trend_a, mut trend_b) = (vec![], vec![]);
    for i in 1..temperature_a.len() {
        if temperature_a[i] - temperature_a[i-1] > 0 {
            trend_a.push(1);
        }
        if temperature_a[i] - temperature_a[i-1] == 0 {
            trend_a.push(0);
        }
        if temperature_a[i] - temperature_a[i-1] < 0 {
            trend_a.push(-1);
        }
        if temperature_b[i] - temperature_b[i-1] > 0 {
            trend_b.push(1);
        }
        if temperature_b[i] - temperature_b[i-1] == 0 {
            trend_b.push(0);
        }
        if temperature_b[i] - temperature_b[i-1] < 0 {
            trend_b.push(-1);
        }
    }
    let mut last_days = 0;
    let mut ret = 0;
    for i in 0..trend_a.len() {
        if trend_a[i] == trend_b[i] {
            last_days += 1;
        } else {
            if last_days > ret {
                ret = last_days
            }
            last_days = 0;
        }
    }
    if last_days > ret {
        ret = last_days
    }
    println!("{:?} {:?} {:?}", trend_a, trend_b, last_days);
    ret
}


fn main() {
    let temperature_a = vec![5,10,16,-6,15,11,3];
    let temperature_b = vec![16,22,23,23,25,3,-16];
    println!("{:?}", temperature_trend(temperature_a, temperature_b));
}
