struct Solution;

impl Solution {
    pub fn construct_rectangle(area: i32) -> Vec<i32> {
        let mut ret = vec!(0; 2);
        let mut cha = 10000000;
        for i in 1..=area/2+1 {
            if area % i == 0 {
                let tmp = area / i;
                let abs = (tmp - i).abs();
                if abs > cha {
                    continue;
                }
                cha = abs;
                if tmp > i {
                    ret[0] = tmp;
                    ret[1] = i;
                } else {
                    ret[0] = i;
                    ret[1] = tmp;
                }
            }
        }
        ret
    }
}

fn main() {
    // println!("{:?}", Solution::construct_rectangle(4));
    // println!("{:?}", Solution::construct_rectangle(37));
    println!("{:?}", Solution::construct_rectangle(9999998));
}
