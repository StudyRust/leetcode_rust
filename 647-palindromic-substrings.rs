pub fn count_substrings(s: String) -> i32 {
    pub fn is_palindromic(cs: &[char]) -> bool {
        // println!("{:?}", cs.clone().to_vec());
        let tmp = cs.clone().to_vec();
        let mut tmp_rev = cs.clone().to_vec();
        tmp_rev.reverse();
        tmp == tmp_rev
    }
    let cs = s.chars().collect::<Vec<char>>();
    let mut ret = 0;
    for i in 1 ..= s.len() {
        for j in 0 ..= (s.len() - i) {
            if is_palindromic(&cs[j..j+i]) { ret += 1 }
        }
    }
    ret
}

fn main() {
    println!("{:?}", count_substrings("abc".to_string()));
    println!("{:?}", count_substrings("aaa".to_string()));
}
