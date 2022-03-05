// 这个题蛮有意思的，可以直接从后遍历，遇到一个数就把所有子集加上该数组成新的子集，遍历完毕即是所有子集
pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = vec!(vec!());
    for num in nums {
        for i in 0..res.len() {                   // 这里只能用index，因为res在循环里面变化
            let mut tmp = res[i].clone();
            tmp.push(num);
            res.push(tmp);
        }
    }
    res
}

fn main() {
    let nums = vec![1,2,3];
    println!("{:?}", subsets(nums));
}
