// backtrace 公式
// result = []
// def backtrack(路径, 选择列表):
//     if 满足结束条件:
//         result.add(路径)
//         return

//     for 选择 in 选择列表:
//         做选择
//         backtrack(路径, 选择列表)
//         撤销选择

struct Solution;

impl Solution {
    pub fn permute(mut path: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ret = vec![];
        Self::backtrack(&mut path, &mut ret, 0);
        ret
    }

    fn backtrack(path: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>, begin: usize) {
        if begin == path.len() {
            ans.push(path.to_vec());
            return;
        }

        for i in begin..path.len() {
            path.swap(i, begin);
            Self::backtrack(path, ans, begin + 1);
            path.swap(i, begin);
        }
    }
}

fn main() {
    let nums = vec![1, 2, 3];
    println!("{:?}", Solution::permute(nums));
}
