impl Solution {
    pub fn count_odds(low: i32, high: i32) -> i32 {
        let mut ret = 0;

        if low % 2 == 1 { ret += 1 }
        if high % 2 == 1 { ret += 1 }

        if ret == 2 {
            ret += (high - low)/2 - 1
        } else {
            ret += (high - low)/2
        }
        ret
    }
}
