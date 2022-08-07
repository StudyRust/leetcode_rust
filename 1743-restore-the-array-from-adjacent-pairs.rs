pub fn restore_array(adjacent_pairs: Vec<Vec<i32>>) -> Vec<i32> {
    use std::collections::HashMap;
    let mut mmp = HashMap::new();
    for p in adjacent_pairs {
        mmp.entry(p[0]).or_insert(Vec::new()).push(p[1]);
        mmp.entry(p[1]).or_insert(Vec::new()).push(p[0]);
    }
    let mut head = 0;
    for (k, v) in &mmp {
        if v.len() == 1 {
            head = *k;
            break;
        }
    }
    // Firstly, find the head or tail
    let mut ret = vec![head];
    let mut count = 1;
    let mut prev = head;
    loop {
        let current = ret.iter().last().unwrap().clone();
        let item = &mmp[&current];
        if item.len() == 1 {
            ret.push(item[0]);
        } else {
            if item[0] == prev {
                ret.push(item[1]);
            } else {
                ret.push(item[0]);
            }
        }
        prev = current;
        count += 1;
        if count == mmp.len() { break }
    }
    ret
}

fn main() {
    let adjacent_pairs = vec![vec![2,1], vec![3,4], vec![3,2]];
    println!("{:?}", restore_array(adjacent_pairs));
}
