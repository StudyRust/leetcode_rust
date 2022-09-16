pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
    use std::collections::HashSet;
    let mut mmp: HashSet<i32> = HashSet::new();
    pub fn dfs(mmp: &mut HashSet<i32>, rooms: &Vec<Vec<i32>>, key: i32) {
        let keys = rooms.get(key as usize);
        if keys == None { return }
        for key in keys.unwrap() {
            let tmp = mmp.get(key);
            if tmp.is_some() { continue }
            mmp.insert(*key);
            dfs(mmp, rooms, *key);
        }
    }
    dfs(&mut mmp, &rooms, 0);
    let mut mmp = mmp.into_iter().collect::<Vec<i32>>();
    mmp.sort();
    (1..rooms.len() as i32).collect::<Vec<i32>>() == mmp
}

fn main() {
    println!("{:?}", can_visit_all_rooms(vec![vec![1], vec![2], vec![3], vec![]]));
    println!("{:?}", can_visit_all_rooms(vec![vec![1,3], vec![3,0,1], vec![2], vec![0]]));
}
