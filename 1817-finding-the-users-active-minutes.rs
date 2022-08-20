pub fn finding_users_active_minutes(logs: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
    use std::collections::HashMap;
    use std::collections::HashSet;
    let mut mmp: HashMap<i32, HashSet<i32>> = HashMap::new();
    for log in logs {
        mmp.entry(log[0]).or_insert(HashSet::new()).insert(log[1]);
    }
    let mut ret = vec![0; k as usize];
    for (_, v) in mmp {
        ret[v.len()-1] += 1;
    }
    ret
}

fn main() {
    let logs: Vec<Vec<i32>> = vec![[0,5],[1,2],[0,2],[0,5],[1,3]].iter().map(|e|e.to_vec()).collect();
    let k = 5;
    println!("{:?}", finding_users_active_minutes(logs, k));
}
