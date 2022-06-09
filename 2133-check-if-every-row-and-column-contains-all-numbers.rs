// using HashSet
pub fn check_valid(matrix: Vec<Vec<i32>>) -> bool {
    use std::collections::HashSet;
    let mut set = HashSet::new();
    for i in 0..matrix.len() {
        set.clear();
        for j in 0..matrix.len() {
            if !set.insert(matrix[i][j]) {
                return false;
            }
        }
    }
    for i in 0..matrix.len() {
        set.clear();
        for j in 0..matrix.len() {
            if !set.insert(matrix[j][i]) {
                return false;
            }
        }
    }
    true
}

fn main() {
    let matrix = vec![vec![1,2,3],vec![3,1,2],vec![2,3,1]];
    println!("{:?}", check_valid(matrix));
}
