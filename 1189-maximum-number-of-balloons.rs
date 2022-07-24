pub fn max_number_of_balloons(text: String) -> i32 {
    let mut stats = vec![0; 5]; // b a l o n
    for c in text.chars().collect::<Vec<char>>() {
        match c {
            'b' => stats[0] += 1,
            'a' => stats[1] += 1,
            'l' => stats[2] += 1,
            'o' => stats[3] += 1,
            'n' => stats[4] += 1,
            _   => {},
        }
    }
    stats[2] /= 2;
    stats[3] /= 2;
    *stats.iter().min().unwrap()
}

fn main() {
    let text = "loonbalxballpoon".to_string();
    println!("{:?}", max_number_of_balloons(text));
}
