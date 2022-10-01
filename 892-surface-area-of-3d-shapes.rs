pub fn surface_area(grid: Vec<Vec<i32>>) -> i32 {
    let mut ret = 0;
    for j in 0..grid.len() {
        for i in 0..grid.len() {
            // 去掉0
            if grid[i][j] == 0 { continue }
            ret += grid[i][j] * 4 + 2;
            // 减去上下左右的紧贴面积
            if i > 0 { ret -= vec![grid[i-1][j], grid[i][j]].iter().min().unwrap() }
            if i+1 < grid.len() { ret -= vec![grid[i+1][j], grid[i][j]].iter().min().unwrap() }
            if j > 0 { ret -= vec![grid[i][j-1], grid[i][j]].iter().min().unwrap() }
            if j+1 < grid.len() { ret -= vec![grid[i][j+1], grid[i][j]].iter().min().unwrap() }
            // println!("{:?}", ret);
        }
    }
    ret
}

fn main() {
    // let grid = vec![vec![1,2],vec![3,4]];
    // println!("{:?}", surface_area(grid));
    let grid = vec![vec![1,1,1],vec![1,0,1],vec![1,1,1]];
    println!("{:?}", surface_area(grid));
}
