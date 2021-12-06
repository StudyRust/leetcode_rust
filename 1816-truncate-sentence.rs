// 示例 1：

// 输入：s = "Hello how are you Contestant", k = 4
// 输出："Hello how are you"
// 解释：
// s 中的单词为 ["Hello", "how" "are", "you", "Contestant"]
// 前 4 个单词为 ["Hello", "how", "are", "you"]
// 因此，应当返回 "Hello how are you"

// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/truncate-sentence
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

pub fn truncate_sentence(s: String, k: i32) -> String {
    let words: Vec<String> = s.split(" ").map(|word| word.to_string()).collect();
    words[0..k as usize].join(" ")
}

fn main() {
    println!("{:?}", truncate_sentence("Hello how are you Contestant".to_string(), 4));
}
