pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
    let mut areas = vec!();
    for x in points.clone() {
        for y in points.clone() {
            for z in points.clone() {
                let area = 0.5 * ((x[0]*y[1] + y[0]*z[1] + z[0]*x[1] - x[0]*z[1] - y[0]*x[1] - z[0]*y[1]) as f64);
                areas.push(area);
            }
        }
    }
    areas.into_iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap()
}

fn main() {
    let points = vec![vec![0,0],vec![0,1],vec![1,0],vec![0,2],vec![2,0]];
    println!("{:?}", largest_triangle_area(points));
}
