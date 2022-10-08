pub fn distance_between_bus_stops(distance: Vec<i32>, start: i32, destination: i32) -> i32 {
    let left = vec![start, destination].into_iter().min().unwrap();
    let right = vec![start, destination].into_iter().max().unwrap();
    vec![distance[left as usize..right as usize].iter().sum::<i32>(),
        distance[0..left as usize].iter().sum::<i32>() +
        distance[right as usize..].iter().sum::<i32>()
    ].into_iter().min().unwrap()
}

fn main() {
    let distance = vec![1, 2, 3, 4];
    let start = 0;
    let destination = 1;
    println!("{:?}", distance_between_bus_stops(distance, start, destination));
    let distance = vec![7,10,1,12,11,14,5,0];
    let start = 7;
    let destination = 2;
    println!("{:?}", distance_between_bus_stops(distance, start, destination));
}
