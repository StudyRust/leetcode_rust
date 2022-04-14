pub fn are_numbers_ascending(s: String) -> bool {
    let mut tmp = 0;
    for token in s.split(" ") {
        match token.parse() {
            Ok(token) => {
                if token <= tmp {
                    return false;
                } else {
                    tmp = token;
                }
            },
            Err(err) => continue,
        };
    }
    true
}

fn main() {
    let s = "1 box has 3 blue 4 red 6 green and 12 yellow marbles".to_string();
    println!("{:?}", are_numbers_ascending(s));
}
