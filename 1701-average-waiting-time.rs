pub fn average_waiting_time(customers: Vec<Vec<i32>>) -> f64 {
    let mut time = 0f64;
    let mut sum = 0f64;
    for customer in &customers {
        if customer[0] as f64 >= time {
            sum += customer[1] as f64;
            time = (customer[0] + customer[1]) as f64;
        } else {
            sum += time + (customer[1] - customer[0]) as f64;
            time += customer[1] as f64;
        }
    }
    sum as f64 / customers.len() as f64
}

fn main() {
    println!("{:?}", average_waiting_time(vec![vec![1,2], vec![2,5], vec![4,3]]));
    println!("{:?}", average_waiting_time(vec![vec![5,2], vec![5,4], vec![10,3], vec![20,1]]));
}
