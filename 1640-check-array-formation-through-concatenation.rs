// 提示：使用Hashmap

pub fn can_form_array(arr: Vec<i32>, pieces: Vec<Vec<i32>>) -> bool {
    use std::collections::HashMap;
    let mut mmp: HashMap<_, _> = HashMap::new();
    let mut ret = vec!();
    for piece in pieces {
        mmp.entry(piece[0]).or_insert(Vec::new()).push(piece);
    }
    for e in &arr {
        match &mmp.get(&e) {
            Some(expr) => {
                for el in &expr[0] {
                    ret.push(*el);
                }
            },
            None => {
                continue;
            },
        }
    }
    arr == ret
}

fn main() {
    let arr = vec![91, 4, 64, 78];
    let pieces = vec![vec![78],vec![4, 64],vec![91]];
    println!("{:?}", can_form_array(arr, pieces));
}
