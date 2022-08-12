pub fn max_ice_cream(costs: Vec<i32>, coins: i32) -> i32 {
    let mut sorted_costs = costs.clone();
    sorted_costs.sort();
    for i in 1..sorted_costs.len() {
        sorted_costs[i] += sorted_costs[i-1];
    }
    for (i, c) in sorted_costs.iter().enumerate() {
        if coins < *c { return i as i32 }
    }
    sorted_costs.len() as i32
}

fn main() {
    let costs = vec![1,6,3,1,2,5];
    let coins = 20;
    println!("{:?}", max_ice_cream(costs, coins));
    println!("{:?}", max_ice_cream(vec![1,3,2,4,1], 7));
}
