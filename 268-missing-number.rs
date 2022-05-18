pub fn missing_number(nums: Vec<i32>) -> i32 {
    let mut n: Vec<_> = (0..=nums.len()).map(usize::from).collect();
    n.retain(|e|!nums.contains(&(*e as i32)));
    *n.first().unwrap() as i32
}

fn main() {
    let nums = vec![3,0,1];
    println!("{:?}", missing_number(nums));
}
