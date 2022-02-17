struct Solution {

}

impl Solution {
    pub fn count_points(rings: String) -> i32 {
        use std::collections::HashSet;

        let mut red: HashSet<char> = HashSet::new();
        let mut green: HashSet<char> = HashSet::new();
        let mut blue: HashSet<char> = HashSet::new();
        let limit_index = &rings.len() / 2;
        let rings_chars: Vec<char> = rings.chars().collect();
        for i in 0..limit_index {
            if rings_chars[2*i] == 'R' {
                red.insert(rings_chars[2*i+1]);
            }
            if rings_chars[2*i] == 'G' {
                green.insert(rings_chars[2*i+1]);
            }
            if rings_chars[2*i] == 'B' {
                blue.insert(rings_chars[2*i+1]);
            }
        }
        let mid: HashSet<_> = red.intersection(&green).collect();
        let mut mid_res: HashSet<char> = HashSet::new();
        for e in mid {
            mid_res.insert(*e);
        }
        let res = blue.intersection(&mid_res);
        res.count() as i32
    }
}

fn main() {
    let rings = "B0B6G0R6R0R6G9".to_string();
    println!("{:?}", Solution::count_points(rings));
}
