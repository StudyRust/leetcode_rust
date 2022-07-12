pub fn is_covered(ranges: Vec<Vec<i32>>, left: i32, right: i32) -> bool {
    let mut ranges = ranges;
    ranges.sort_by(|a, b|a[0].cmp(&b[0]));
    let mut stack: Vec<i32> = (left..=right).rev().collect();
    let mut current = stack.pop().unwrap();
    for range in &ranges {
        while current >= range[0] && current <= range[1] {
            if stack.is_empty() {
                return true;
            } else {
                current = stack.pop().unwrap();
            }
        }
    }
    false
}

fn main() {
    let ranges = [[36,50],[14,28],[4,31],[24,37],[13,36],[27,33],[23,32],[23,27],[1,35]].map(|e|e.to_vec()).to_vec();
    let left = 35;
    let right = 40;
    println!("{:?}", is_covered(ranges, left, right));
}
