pub fn two_edit_words(queries: Vec<String>, dictionary: Vec<String>) -> Vec<String> {
    let dic_chars = dictionary.iter().map(|s|s.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut queries = queries;
    queries.retain(|s|{
        'outer: for d_ch in &dic_chars {
            let mut count = 0;
            for (i, c) in s.chars().collect::<Vec<char>>().iter().enumerate() {
                if d_ch[i] != *c { count += 1 }
                if count > 2 { continue 'outer }
            }
            return true
        }
        return false
    });
    queries
}

fn main() {
    let queries = vec!["word".to_string(), "note".to_string(),
        "ants".to_string(), "wood".to_string()];
    let dictionary = vec!["wood".to_string(),
        "joke".to_string(), "moat".to_string()];
    println!("{:?}", two_edit_words(queries, dictionary));
    let queries = vec!["yes".to_string()];
    let dictionary = vec!["not".to_string()];
    println!("{:?}", two_edit_words(queries, dictionary));
}
