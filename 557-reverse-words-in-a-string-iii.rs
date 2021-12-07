// 示例：

// 输入："Let's take LeetCode contest"
// 输出："s'teL ekat edoCteeL tsetnoc"

pub fn reverse_words(s: String) -> String {
    s.split(" ").map(|word|word.to_string().chars().rev().collect()).collect::<Vec<String>>().join(" ")
}

fn main() {
    println!("{:?}", reverse_words("Let's take LeetCode contest".to_string()));
}
