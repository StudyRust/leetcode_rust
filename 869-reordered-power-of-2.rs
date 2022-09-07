// Enumerate numbers with same length

pub fn reordered_power_of2(n: i32) -> bool {
    let mut candidate_numbers = vec![];
    let n_len = n.to_string().len();
    let mut power = 0;
    loop {
        let tmp = 2_i32.pow(power);
        if tmp.to_string().len() == n_len {
            candidate_numbers.push(tmp);
        }
        if tmp.to_string().len() > n_len {
            break;
        }
        power += 1
    }
    let mut sorted_nums = n.to_string().chars().collect::<Vec<char>>();
    sorted_nums.sort();
    for number in &candidate_numbers {
        let mut sorted_tmp = number.to_string().chars().collect::<Vec<char>>();
        sorted_tmp.sort();
        if sorted_tmp == sorted_nums {
            return true;
        }
    }
    false
}

fn main() {
    println!("{:?}", reordered_power_of2(10));
}
