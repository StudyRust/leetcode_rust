struct Solution;

// 回溯 = 递归 + 循环
impl Solution {
    pub fn backtrace(n: i32, k: usize, start_index: i32, path: &mut Vec<i32>, ret: &mut Vec<Vec<i32>>) {
        if path.len() == k {
            ret.push(path.to_vec());
            return;
        }
        for i in start_index..=n {
            path.push(i);
            Self::backtrace(n, k, i+1, path, ret);
            path.pop();
        }
    }

    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut path = vec![];
        let mut ret = vec![];
        Self::backtrace(n, k as usize, 1, &mut path, &mut ret);
        ret
    }
}

fn main() {
    println!("{:?}", Solution::combine(4, 2));
}
