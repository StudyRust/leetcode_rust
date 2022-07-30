pub fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
    let mut flattened = mat.clone().into_iter().flatten().collect::<Vec<i32>>();
    if flattened.len() != (r  * c) as usize { return mat }
    let mut ret = vec![];
    for _ in 0..r {
        let mut tmp = vec![];
        for _ in 0..c {
            tmp.push(flattened.remove(0));
        }
        ret.push(tmp);
    }
    ret
}

fn main() {
    let mat = vec![vec![1,2], vec![3,4]];
    let r = 1;
    let c = 4;
    println!("{:?}", matrix_reshape(mat, r, c));
}
