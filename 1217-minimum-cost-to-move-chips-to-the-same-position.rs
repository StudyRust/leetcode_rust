// 因为移动2个位置不需要代价，那么奇数位置移到奇数位置不用代价，
// 偶数位置移到偶数位置不用代价，那就分别统计奇数位置和偶数位置的个数，
// 相当于把所有奇数放一起，所有偶数的放一起，然后比较奇数的少还是偶数的少，
// 将少的个数移到多的个数位置上去就可以了12.

pub fn min_cost_to_move_chips(position: Vec<i32>) -> i32 {
    let mut count = vec!(0; 2);
    for p in position {
        if p % 2 == 0 {
            count[0] += 1;
        } else {
            count[1] += 1;
        }
    }
    *count.iter().min().unwrap()
}

fn main() {
    let position = vec![2,3,2,3,2];
    println!("{:?}", min_cost_to_move_chips(position));
}
