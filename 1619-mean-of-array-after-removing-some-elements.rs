pub fn trim_mean(arr: Vec<i32>) -> f64 {
    let length = arr.len();
    let offset = (length as f64 * 0.05) as usize;
    let mut sorted_arr = arr.clone();
    sorted_arr.sort();
    let res = sorted_arr[offset..(length-offset)].iter().sum::<i32>() as f64 / (length-offset*2) as f64;
    format!("{:.5}", res).parse().unwrap()
}

fn main() {
    let arr = vec![6,0,7,0,7,5,7,8,3,4,0,7,8,1,6,8,1,1,2,4,8,1,9,5,4,3,8,5,10,8,6,6,1,0,6,10,8,2,3,4];
    println!("{:?}", trim_mean(arr));
}
