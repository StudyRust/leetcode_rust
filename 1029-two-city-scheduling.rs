pub fn two_city_sched_cost(costs: Vec<Vec<i32>>) -> i32 {
    let n = costs.len();
    let mut sorted_costs = costs.clone();
    sorted_costs.sort_by(|a, b|(a[0]-a[1]).cmp(&(b[0]-b[1])));
    &sorted_costs[0..n/2].iter().map(|e|e[0]).sum::<i32>() + &sorted_costs[n/2..].iter().map(|e|e[1]).sum::<i32>()
}

fn main() {
    let costs = vec![[10,20],[30,200],[400,50],[30,20]].iter().map(|e|e.to_vec()).collect();
    println!("{:?}", two_city_sched_cost(costs));
}
