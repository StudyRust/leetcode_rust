// 计算点到圆心的距离
pub fn count_points(points: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut ret = vec!();
    for q in queries {
        let mut count = 0;
        for p in &points {
            let dis = (((p[0]-q[0]) as f64).powf(2.0) + ((p[1]-q[1]) as f64).powf(2.0)).sqrt();
            if dis <= q[2] as f64 {
                count += 1;
            }
        }
        ret.push(count);
    }
    ret
}

fn main() {
    let points = vec![vec![1,3],vec![3,3],vec![5,3],vec![2,2]];
    let queries = vec![vec![2,3,1],vec![4,3,1],vec![1,1,2]];
    println!("{:?}", count_points(points, queries));
}
