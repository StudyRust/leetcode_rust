pub fn lucky_numbers (matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let mut res = Vec::new();
    'outer: for row in &matrix {
        let row_min = row.iter().min().unwrap();
        let i = row.iter().position(|&r| r == *row_min).unwrap();
        for r in &matrix {
            if r[i] > *row_min {
                continue 'outer;
            }
        }
        res.push(*row_min);
    }
    res
}

fn main() {
    let matrix = vec!(vec!(3,7,8),vec!(9,11,13),vec!(15,16,17));
    println!("{:?}", lucky_numbers(matrix));
}
