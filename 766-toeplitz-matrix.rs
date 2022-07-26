pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
    let (m, n) = (matrix[0].len(), matrix.len());
    // 00 11 22
    // 01 12 23
    // 02 13
    // 03
    for y in 0..m {
        let mut i = 0;
        let mut j = y;
        let prev = matrix[i][j];
        while j < m && i < n {
            if matrix[i][j] != prev { return false; }
            i += 1;
            j += 1;
        }
    }
    // 00 11 22
    // 10 21
    // 20
    for x in 0..n {
        let mut i = x;
        let mut j = 0;
        let prev = matrix[i][j];
        while j < m && i < n {
            if matrix[i][j] != prev { return false; }
            i += 1;
            j += 1;
        }
    }
    true
}

fn main() {
    let matrix = vec![vec![1,2,3,4], vec![5,1,2,3], vec![9,5,1,2]];
    println!("{:?}", is_toeplitz_matrix(matrix));
}
