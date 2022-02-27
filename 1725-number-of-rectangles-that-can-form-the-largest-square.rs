pub fn count_good_rectangles(rectangles: Vec<Vec<i32>>) -> i32 {
    use std::collections::HashMap;
    let mut mmp = HashMap::new();
    for rectangle in rectangles {
        let min: i32 = *rectangle.iter().min().unwrap();
        let count = mmp.entry(min).or_insert(0);
        *count += 1;
    }
    let mut count_vec: Vec<_> = mmp.iter().collect();
    count_vec.sort_by(|a, b|a.0.cmp(b.0));
    *count_vec.iter().last().unwrap().1
}

fn main() {
    let rectangles = vec![vec![5,8],vec![3,9],vec![5,12],vec![16,5]];
    println!("{:?}", count_good_rectangles(rectangles));
}
