pub fn fair_candy_swap(alice_sizes: Vec<i32>, bob_sizes: Vec<i32>) -> Vec<i32> {
    let diff: i32 = alice_sizes.iter().sum::<i32>() - bob_sizes.iter().sum::<i32>();
    let abs_diff = diff / 2;
    let mut ret = vec![];
    'o: for i in &alice_sizes {
        for j in &bob_sizes {
            if abs_diff == *i - *j {
                ret = vec![*i, *j];
                break 'o
            }
        }
    }
    ret
}

fn main() {
    println!("{:?}", fair_candy_swap(vec![1, 1], vec![2, 2]));
    println!("{:?}", fair_candy_swap(vec![1, 2], vec![2, 3]));
    println!("{:?}", fair_candy_swap(vec![2], vec![1, 3]));
    println!("{:?}", fair_candy_swap(vec![1, 2, 5], vec![2, 4]));
}
