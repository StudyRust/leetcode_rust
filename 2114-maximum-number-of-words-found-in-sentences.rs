pub fn most_words_found(sentences: Vec<String>) -> i32 {
    let arr: Vec<usize> = sentences.iter().map(|e|e.split(" ").count()).collect();
    *arr.iter().max().unwrap() as i32
}

fn main() {
    let sentences: Vec<String> = ["alice and bob love leetcode", "i think so too", "this is great thanks very much"]
        .iter().map(|e|e.to_string()).collect();
    println!("{:?}", most_words_found(sentences));
}
