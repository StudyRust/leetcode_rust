pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let mut ret = vec!();
    'outer: for (i, t) in temperatures.iter().enumerate() {
        for (j, e) in temperatures[i+1..].iter().enumerate() {
            if *e > *t {
                ret.push(j as i32 + 1);
                continue 'outer;
            }
        }
        ret.push(0);
    }
    ret
}

fn main() {
    let temperatures = vec![73,74,75,71,69,72,76,73];
    println!("{:?}", daily_temperatures(temperatures));
}
