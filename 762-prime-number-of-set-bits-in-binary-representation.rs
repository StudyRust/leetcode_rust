pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
    pub fn is_prime(num: usize) -> bool {
        if num < 2 {
            return false;
        }
        for i in 2..num {
            if num % i == 0 {
                return false;
            }
        }
        return true;
    }
    let mut ret = 0;
    for num in left..=right {
        let num_binary: Vec<char> = format!("{:b}", num).chars().collect();
        let count_binary = num_binary.iter().filter(|&n|*n=='1').count();
        if is_prime(count_binary) {
            ret += 1;
        }
    }
    ret
}

fn main() {
    let (left, right) = (10, 15);
    println!("{:?}", count_prime_set_bits(left, right));
}
