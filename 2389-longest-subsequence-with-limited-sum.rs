pub fn answer_queries(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
    fn cal(q: i32, sorted_nums: &[i32]) -> i32 {
        let mut sum = 0;
        for (i, n) in sorted_nums.iter().enumerate() {
            sum += n;
            if sum > q {
                return i as i32
            }
        }
        sorted_nums.len() as i32
    }
    let mut sorted_nums = nums.clone();
    sorted_nums.sort();
    queries.iter().map(|e|cal(*e, &sorted_nums)).collect()
}

fn main() {
    let nums = vec![4,5,2,1];
    let queries = vec![3,10,21];
    println!("{:?}", answer_queries(nums, queries));
}
