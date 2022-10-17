pub fn sum_even_after_queries(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut ret = vec![];
    let mut nums = nums.clone();
    for i in 0 .. nums.len() {
        let val = queries[i][0];
        let index = queries[i][1];
        nums[index as usize] += val;
        let mut tmp = 0;
        for j in 0 .. nums.len() {
            if nums[j] % 2 == 0 { tmp += nums[j] }
        }
        ret.push(tmp)
    }
    ret
}

fn main() {
    let nums = vec![1,2,3,4];
    let queries = vec![vec![1,0], vec![-3,1], vec![-4,0], vec![2,3]];
    println!("{:?}", sum_even_after_queries(nums, queries));
}
