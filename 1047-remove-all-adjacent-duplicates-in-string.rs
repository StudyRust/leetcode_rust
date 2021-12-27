fn main() {
    println!("{}", remove_duplicates("abbaca".to_string()));
}

pub fn remove_duplicates(s: String) -> String {
    let mut stack = Vec::new();
    for c in s.chars() {
        if stack.is_empty() {
            stack.push(c);
        } else {
            if c != *stack.last().unwrap() {
                stack.push(c);
            } else {
                stack.pop();
            }
        }
    }
    stack.into_iter().collect()
}
