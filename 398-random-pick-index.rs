use std::collections::HashMap;

struct Solution {
    mmp: HashMap<i32, Vec<usize>>
}

impl Solution {

    fn new(nums: Vec<i32>) -> Self {
        let mut mmp = HashMap::new();
        for (i, num) in nums.iter().enumerate() {
            mmp.entry(*num).or_insert(Vec::new()).push(i);
        }
        Solution {
            mmp: mmp
        }
    }

    fn pick(&mut self, target: i32) -> i32 {
        self.mmp.get_mut(&target).unwrap().rotate_right(1);
        self.mmp[&target][0] as i32
    }
}

fn main() {
    let mut obj = Solution::new(vec![1, 2, 3, 3, 3]);
    println!("{:?}", obj.pick(3));
    println!("{:?}", obj.pick(1));
    println!("{:?}", obj.pick(3));
    println!("{:?}", obj.pick(3));
    println!("{:?}", obj.pick(3));
    println!("{:?}", obj.pick(3));
    println!("{:?}", obj.pick(3));
    println!("{:?}", obj.pick(3));
    println!("{:?}", obj.pick(3));
    println!("{:?}", obj.pick(3));
}
