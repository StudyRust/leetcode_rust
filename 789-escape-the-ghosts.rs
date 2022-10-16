// 这题考阅读理解和题意转化能力，其实问的就是有没鬼能比你更快到达终点(转)
pub fn escape_ghosts(ghosts: Vec<Vec<i32>>, target: Vec<i32>) -> bool {
    let dis = target[0].abs() + target[1].abs();
    for g in ghosts {
        if (g[0] - target[0]).abs() + (g[1] - target[1]).abs() <= dis {
            return false
        }
    }
    true
}

fn main() {
    println!("{:?}", escape_ghosts(vec![vec![1,0], vec![0,3]], vec![0,1]));
    println!("{:?}", escape_ghosts(vec![vec![2,0]], vec![1,0]));
    println!("{:?}", escape_ghosts(vec![vec![2,0]], vec![1,0]));
}
