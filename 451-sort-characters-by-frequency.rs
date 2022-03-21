pub fn frequency_sort(s: String) -> String {
    use std::collections::HashMap;
    let mut count_map: HashMap<char, Vec<char>> = HashMap::new();
    for c in s.chars() {
        count_map.entry(c).or_insert(Vec::new()).push(c);
    }
    let mut count_vec: Vec<_> = count_map.iter().collect();
    count_vec.sort_by(|b, a|a.1.len().cmp(&b.1.len()));
    let mut res = String::new();
    for e in count_vec {
        for c in e.1 {
            res.push(*c);
        }
    }
    res
}

fn main() {
    let s = "Aabb".to_string();
    println!("{:?}", frequency_sort(s));
}
