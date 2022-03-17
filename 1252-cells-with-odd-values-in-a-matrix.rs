pub fn odd_cells(m: i32, n: i32, indices: Vec<Vec<i32>>) -> i32 {
    // let v = vec![0; 10]; // ten zeroes
    // 用以上方法构造矩阵
    // [[0, 0, 0], [0, 0, 0]]
    let mut a = vec![vec![0; n as usize]; m as usize];
    for index in indices {
        for i in 0..n {
            a[index[0] as usize][i as usize] += 1;
        }
        for j in 0..m {
            a[j as usize][index[1] as usize] += 1;
        }
    }
    let mut counter = 0;
    for row in a {
        for e in row {
            if e % 2 != 0{
                counter += 1
            }
        }
    }
    counter
}

fn main() {
    let (m, n) = (2, 2);
    let indices = vec![vec![1,1],vec![0,0]];
    println!("{:?}", odd_cells(m, n, indices));
}
