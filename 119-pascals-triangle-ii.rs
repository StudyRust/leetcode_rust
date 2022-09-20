pub fn get_row(row_index: i32) -> Vec<i32> {
    let mut ret = vec![vec![1], vec![1, 1]];
    for i in 2..=row_index as usize {
        let mut tmp = vec![];
        for j in 1..ret[i-1].len() {
            tmp.push(ret[i-1][j] + ret[i-1][j-1])
        }
        tmp.push(1);
        tmp.insert(0, 1);
        ret.push(tmp);
    }
    ret[row_index as usize].clone()
}

fn main() {
    println!("{:?}", get_row(0));
    println!("{:?}", get_row(1));
    println!("{:?}", get_row(2));
    println!("{:?}", get_row(3));
}
