pub fn find_the_distance_value(arr1: Vec<i32>, arr2: Vec<i32>, d: i32) -> i32 {
    let mut res = 0;
    'outer: for num in arr1 {
        for e in &arr2 {
            if (num - e).abs() <= d {
                continue 'outer;
            }
        }
        res += 1;
    }
    res
}

fn main() {
    let arr1 = vec![4,5,8];
    let arr2 = vec![10,9,1,8];
    let d = 2;
    println!("{:?}", find_the_distance_value(arr1, arr2, d));
}
