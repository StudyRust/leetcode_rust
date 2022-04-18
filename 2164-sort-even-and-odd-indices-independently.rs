pub fn sort_even_odd(nums: Vec<i32>) -> Vec<i32> {
    let mut odd_arr = vec!();
    let mut even_arr = vec!();
    let mut ret = vec!();
    let half_len = nums.len() / 2 + 1;
    for i in 0..half_len {
        if nums.get(2*i).is_some() {
            even_arr.push(*nums.get(2*i).unwrap());
        }
        if nums.get(2*i+1).is_some() {
            odd_arr.push(*nums.get(2*i+1).unwrap());
        }
    }
    odd_arr.sort_by(|a, b|b.cmp(a));
    even_arr.sort();
    // println!("{:?} {:?}", odd_arr, even_arr);
    for i in 0..half_len {
        if even_arr.get(i).is_some() {
            ret.push(*even_arr.get(i).unwrap());
        }
        if odd_arr.get(i).is_some() {
            ret.push(*odd_arr.get(i).unwrap());
        }
    }
    ret
}

fn main() {
    let nums = vec![5,39,33,5,12,27,20,45,14,25,32,33,30,30,9,14,44,15,21]
;
    println!("{:?}", sort_even_odd(nums));
}
