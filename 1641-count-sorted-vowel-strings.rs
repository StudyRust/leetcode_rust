// 数学，推倒完，公式是 C(n+4, 4)
pub fn count_vowel_strings(n: i32) -> i32 {
    (n+4)*(n+3)*(n+2)*(n+1)/24
}

fn main() {
    println!("{:?}", count_vowel_strings(2));
}
