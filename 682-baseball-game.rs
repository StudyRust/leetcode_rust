pub fn cal_points(ops: Vec<String>) -> i32 {
    let mut arr = vec!();
    for e in ops {
        if e == "C".to_string() {
            arr.pop();
        } else if e == "D".to_string() {
            let tmp = arr.iter().last().unwrap() * 2;
            arr.push(tmp);
        } else if e == "+".to_string() {
            let start = &arr.len() - 2;
            arr.push(arr[start..].iter().sum::<i32>());
        } else {
            let num: i32 = e.parse().unwrap();
            arr.push(num);
        }
    }
    arr.iter().sum()
}

fn main() {
    let ops = vec!["5","-2","4","C","D","9","+","+"].iter().map(|e|e.to_string()).collect();
    println!("{:?}", cal_points(ops));
}
