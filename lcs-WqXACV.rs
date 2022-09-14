pub fn half_questions(questions: Vec<i32>) -> i32 {
    let n = questions.len() / 2;
    use std::collections::HashMap;
    let mut mmp = HashMap::new();
    for i in questions { *mmp.entry(i).or_insert(0) += 1 }
    let mut mmp_vec: Vec<_> = mmp.iter().collect();
    mmp_vec.sort_by(|b, a|a.1.cmp(&b.1));
    let mut ret = 0;
    let mut current = 0;
    for e in mmp_vec {
        ret += 1;
        current += e.1;
        if current >= n { break }
    }
    ret
}

fn main() {
    println!("{:?}", half_questions(vec![13,8,3,7,5,6,11,12,3,6,6,11]));
    println!("{:?}", half_questions(vec![2,1,6,2]));
    println!("{:?}", half_questions(vec![1,5,1,3,4,5,2,5,3,3,8,6]));
}
