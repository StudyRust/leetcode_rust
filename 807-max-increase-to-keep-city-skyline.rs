pub fn max_increase_keeping_skyline(grid: Vec<Vec<i32>>) -> i32 {
    let mut res = 0;
    let mut cols_max = vec!();
    let index = grid.len();
    for i in 0..index {
        let mut tmp = vec!();
        for j in 0..index {
            tmp.push(grid[j][i]);
        }
        let col_max = tmp.into_iter().max().unwrap();
        cols_max.push(col_max);
    }
    for row in grid {
        let row_max = row.iter().max().unwrap();
        let mut added: i32;
        for (i, e) in row.iter().enumerate() {
            if cols_max[i] < *row_max {
                added = cols_max[i] - e;
            } else {
                added = row_max - e;
            }
            res += added;
        }
    }
    res
}

fn main() {
    let grid = vec![vec![3,0,8,4],vec![2,4,5,7],vec![9,2,6,3],vec![0,3,1,0]];
    println!("{:?}", max_increase_keeping_skyline(grid));
}
