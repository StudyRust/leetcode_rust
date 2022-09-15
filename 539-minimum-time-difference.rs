pub fn find_min_difference(time_points: Vec<String>) -> i32 {
    let mut mmp = vec![];
    for p in time_points {
        let arr = p.split(":").map(|e|e.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        mmp.push(arr[0]*60+arr[1]);
        mmp.push((arr[0]+24)*60+arr[1]);
    }
    mmp.sort();
    let mut ret = 24 * 60;
    for i in 1..mmp.len() {
        let tmp = mmp[i] - mmp[i-1];
        if tmp < ret { ret = tmp }
    }
    ret
}

fn main() {
    println!("{:?}", find_min_difference(vec!["00:00".to_string(),"23:59".to_string(),"00:00".to_string()]));
}
