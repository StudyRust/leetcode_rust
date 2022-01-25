pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
    let mut sorted_score = score.clone();
    sorted_score.sort_by(|a, b|b.cmp(a));
    use std::collections::HashMap;
    let mut mmp = HashMap::new();
    for (i, s) in sorted_score.iter().enumerate() {
        let v: String;
        let j = i + 1;
        if j == 1 {
            v = "Gold Medal".to_string();
        } else if j == 2 {
            v = "Silver Medal".to_string();
        } else if j == 3 {
            v = "Bronze Medal".to_string();
        } else {
            v = j.to_string();
        }
        mmp.insert(s, v);
    }
    let res: Vec<String> = score.into_iter().map(|e|mmp[&e].clone()).collect();
    res
}

fn main() {
    let score = vec![5,4,3,2,1];
    println!("{:?}", find_relative_ranks(score));
}