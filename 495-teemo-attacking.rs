// (1...time_series.length).map{|i|[duration, (time_series[i]-time_series[i-1])].min}.sum+duration

pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
    (1..time_series.len()).map(|i|vec![duration, (time_series[i]-time_series[i-1])].into_iter().min().unwrap()).sum::<i32>()+duration
}

fn main() {
    let time_series = vec![1,4];
    let duration = 2;
    println!("{:?}", find_poisoned_duration(time_series, duration));
}
