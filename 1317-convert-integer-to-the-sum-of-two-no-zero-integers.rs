pub fn get_no_zero_integers(n: i32) -> Vec<i32> {
    pub fn is_contain_zero(n: i32) -> bool {
        n.to_string().chars().collect::<Vec<char>>().contains(&'0')
    }
    let mut ret = vec!();
    for i in 1..=n/2 {
        if !is_contain_zero(i) && !is_contain_zero(n-i) {
            ret.push(i);
            ret.push(n-i);
            break;
        }
    }
    ret
}

fn main() {
    let n = 2;
    println!("{:?}", get_no_zero_integers(n));
}
