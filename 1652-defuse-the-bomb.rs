pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
    if k == 0 { return vec![0; code.len()] }
    let mut ret = vec![];
    for i in 0..code.len() {
        if k > 0 {
            let tail = i + k as usize;
            if tail <= code.len() - 1 {
                ret.push(code[i+1..=tail].into_iter().sum::<i32>());
            } else {
                let tmp: i32 = &code[i+1..].into_iter().sum::<i32>() +
                        &code[0..tail-code.len()+1].into_iter().sum::<i32>();
                ret.push(tmp);
            }
        } else {
            let head = i as i32 + k; // i =1 k = -2
            if head >= 0 {
                ret.push(code[head as usize..i].into_iter().sum::<i32>());
            } else {
                let tmp: i32 = &code[0..i].into_iter().sum::<i32>() +
                        &code[(head + code.len() as i32) as usize..].into_iter().sum::<i32>();
                ret.push(tmp);
            }
        }
    }
    ret
}

fn main() {
    println!("{:?}", decrypt(vec![5, 7, 1, 4], 3));
    println!("{:?}", decrypt(vec![2, 4, 9, 3], -2));
}
