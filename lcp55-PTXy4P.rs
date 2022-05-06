struct Solution;

impl Solution {
    pub fn get_minimum_time(time: Vec<i32>, fruits: Vec<Vec<i32>>, limit: i32) -> i32 {
        // tranlate from Ruby to Rust code
        // fruits.map{|e|time[e[0]]*(e[1]/limit.to_f).ceil}.sum
        fruits.iter().map(|e|time[e[0] as usize]*(e[1] as f64 / limit as f64).ceil() as i32).sum()
    }
}

fn main() {
    let time = vec![2,3,2];
    let fruits = vec![vec![0,2], vec![1,4], vec![2,1]];
    let limit = 3;
    println!("{:?}", Solution::get_minimum_time(time, fruits, limit));
}
