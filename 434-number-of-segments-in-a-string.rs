pub fn count_segments(s: String) -> i32 {
    s.split_ascii_whitespace().count() as i32
}

fn main() {
    println!("{:?}", count_segments("s Hello, my name is John".to_string()));
}
