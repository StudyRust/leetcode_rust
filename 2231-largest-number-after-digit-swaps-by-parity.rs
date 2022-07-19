pub fn largest_integer(num: i32) -> i32 {
    let mut num_digits = vec!();
    let mut num = num;
    while num > 0 {
        num_digits.insert(0, num % 10);
        num /= 10;
    }
    let num_digits_len = num_digits.len();
    let mut odd_idx = vec!();
    let mut odd_num = vec!();
    let mut even_idx = vec!();
    let mut even_num = vec!();
    for (i, d) in num_digits.iter().enumerate() {
        if d % 2 == 0 {
            even_idx.push(i);
            even_num.push(d);
        } else {
            odd_idx.push(i);
            odd_num.push(d);
        }
    }
    odd_num.sort_by(|a, b|b.cmp(&a));
    even_num.sort_by(|a, b|b.cmp(&a));
    let mut ret_digits = vec!(0; num_digits_len);
    for (i, d) in odd_idx.iter().enumerate() {
        ret_digits[*d] = *odd_num[i];
    }
    for (i, d) in even_idx.iter().enumerate() {
        ret_digits[*d] = *even_num[i];
    }
    let mut ret = 0;
    for (i, d) in ret_digits.iter().rev().enumerate() {
        ret += d * 10_i32.pow(i as u32);
    }
    ret
}

fn main() {
    println!("{:?}", largest_integer(65875));
}
