pub fn intersection(nums: Vec<Vec<i32>>) -> Vec<i32> {
    use std::collections::HashSet;
    let mut ret: HashSet<i32> = HashSet::new();
    for i in &nums[0] {
        ret.insert(*i);
    }
    for a in &nums[1..] {
        let mut tmp: HashSet<i32> = HashSet::new();
        for i in a {
            tmp.insert(*i);
        }
        let mut t: HashSet<i32> = HashSet::new();
        for x in ret.intersection(&tmp) {
            t.insert(*x);
        }
        ret = t;
    }
    let mut ret = ret.into_iter().collect::<Vec<i32>>();
    ret.sort();
    ret
}

fn main() {
    let nums = vec![vec![3,1,2,4,5], vec![1,2,3,4], vec![3,4,5,6]];
    println!("{:?}", intersection(nums));
}
