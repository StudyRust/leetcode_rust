pub fn fill_cups(amount: Vec<i32>) -> i32 {
    let mut sort_am = amount.clone();
    sort_am.sort();
    if sort_am[2] >= sort_am[0] + sort_am[1] {
        // return amount[0] + amount[1] + amount[2] - (amount[0] + amount[1])
        sort_am[2]
    } else {
        let mut ret = sort_am[2];
        let left = sort_am[0] + sort_am[1] - sort_am[2];
        if left % 2 == 0 {
            ret += left / 2
        } else {
            ret += left / 2 + 1
        }
        ret
    }
}

fn main() {
    println!("{:?}", fill_cups(vec![1,4,2]));
    println!("{:?}", fill_cups(vec![5,4,4]));
    println!("{:?}", fill_cups(vec![3,4,5]));
    println!("{:?}", fill_cups(vec![5,0,0]));
}
