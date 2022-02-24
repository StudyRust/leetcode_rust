pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let times = matrix[0].len();
    let mut res = vec!();
    for i in 0..times {
        let mut tmp = vec!();
        for e in &matrix {
            tmp.push(e[i]);
        }
        res.push(tmp);
    }
    res
}

fn main() {
    let matrix = vec![vec![1,2,3],vec![4,5,6]];
    println!("{:?}", transpose(matrix));
}
