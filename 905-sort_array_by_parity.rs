pub fn sort_array_by_parity(nums: Vec<i32>) -> Vec<i32> {
    let mut res = Vec::new();
    let mut vec = nums.clone();
    vec.retain(|&x| x % 2 == 0);
    res.append(&mut vec);
    let mut vec = nums.clone();
    vec.retain(|&x| x % 2 != 0);
    res.append(&mut vec);
    res
}

fn main() {
    println!("{:?}", sort_array_by_parity(vec![3,1,2,4]));
}
