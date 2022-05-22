pub fn is_anagram(s: String, t: String) -> bool {
    let mut a = s.chars().collect::<Vec<char>>();
    let mut b = t.chars().collect::<Vec<char>>();
    a.sort();
    b.sort();
    a == b
}

fn main() {
    let s = "rat".to_string();
    let t = "car".to_string();
    println!("{:?}", is_anagram(s, t));
}
