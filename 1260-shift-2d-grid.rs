pub fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let grid_len = grid[0].len();
    let mut a = grid.into_iter().flatten().collect::<Vec<i32>>();
    let a_len = a.len();
    a.rotate_right(k as usize % a_len);
    a.chunks(grid_len).into_iter().map(|e|e.to_vec()).collect()
}

fn main() {
    let grid = vec![vec![3,8,1,9],vec![19,7,2,5],vec![4,6,11,10],vec![12,0,21,13]];
    let k = 4;
    println!("{:?}", shift_grid(grid, k));
    let grid = vec![vec![1]];
    let k = 100;
    println!("{:?}", shift_grid(grid, k));
}
