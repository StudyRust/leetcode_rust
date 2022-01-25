pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
    let bits_strs: Vec<String> = bits.into_iter().map(|b|b.to_string()).collect::<Vec<String>>();
    let bits_str = bits_strs.join("");
    if bits_str == "0".to_string() || bits_str == "00".to_string() {
        true
    } else {
        bits_str.ends_with("100") ||
        bits_str.ends_with("000") ||
        ( bits_str.ends_with("110") && !bits_str.ends_with("1110")) ||
        ( bits_str.ends_with("11110") && !bits_str.ends_with("111110"))
    }
}

fn main() {
    let bits = vec![1,1,1,1,0];
    println!("{:?}", is_one_bit_character(bits));
}
