pub fn reverse_vowels(s: String) -> String {
    let cs = s.chars().collect::<Vec<char>>();
    let mut cs_clone = cs.clone();
    let vowels = vec!('a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U');
    let mut vowels_idx = vec!();
    let mut vowels_rev = vec!();
    for (i, c) in cs.iter().enumerate() {
        if vowels.contains(&c) {
            vowels_idx.push(i);
            vowels_rev.push(c);
        }
    }
    vowels_rev.reverse();
    for (i, c) in vowels_rev.iter().enumerate() {
        cs_clone[vowels_idx[i]] = **c;
    }
    cs_clone.iter().collect()
}

fn main() {
    println!("{:?}", reverse_vowels("leetcode".to_string()));
    println!("{:?}", reverse_vowels("hello".to_string()));
}
