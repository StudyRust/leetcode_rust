pub fn count_good_triplets(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32 {
    let mut ret = 0;
    for i in 0..arr.len() {
        for j in (i+1)..arr.len() {
            for k in (j+1)..arr.len() {
                if (arr[i] - arr[j]).abs() <= a &&
                    (arr[j] - arr[k]).abs() <= b &&
                    (arr[i] - arr[k]).abs() <= c {
                    ret += 1;
                }
            }
        }
    }
    ret
}

fn main() {
    let arr = vec![3,0,1,1,9,7];
    let (a, b, c) = (7, 3, 2);
    println!("{:?}", count_good_triplets(arr, a, b, c));
}
