pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
    let mut ret = 0;
    let mut ps = vec![];
    for i in 0..mat.len() {
        for j in 0..mat[0].len() {
            ps.push((i, j));
        }
    }
    for p in ps {
        if mat[p.0][p.1] == 1 {
            let mut col_count = 0;
            let mut row_count = 0;
            for e in &mat[p.0] {
                if *e == 1 { row_count += 1 }
            }
            for i in 0..mat.len() {
                if mat[i][p.1] == 1 { col_count += 1 }
            }
            if col_count == 1 && row_count == 1 { ret += 1 }
        }
    }
    ret
}

fn main() {
    let mat = vec![
                vec![0,0],
                vec![0,0],
                vec![1,0]
              ];
    println!("{:?}", num_special(mat));
}
