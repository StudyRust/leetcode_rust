// 为了方便，所有 26 个英文字母的摩尔斯密码表如下：

// [".-","-...","-.-.","-..",".","..-.","--.","....","..",".---","-.-",".-..","--","-.","---",".--.","--.-",".-.","...","-","..-","...-",".--","-..-","-.--","--.."]

// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/unique-morse-code-words
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

pub fn unique_morse_representations(words: Vec<String>) -> i32 {
    let mima: Vec<String> = [".-","-...","-.-.","-..",".","..-.","--.","....","..",".---","-.-",".-..","--","-.","---",".--.","--.-",".-.","...","-","..-","...-",".--","-..-","-.--","--.."].iter().map(|&s|s.into()).collect();
    let mut mima_words = vec!();
    for word in words {
        let mut mima_word = "".to_string();
        for c in word.chars() {
            mima_word.push_str(&mima[c as usize - 'a' as usize]);
        }
        mima_words.push(mima_word);
    }
    mima_words.sort_unstable();
    mima_words.dedup();
    mima_words.len() as i32
}

fn main() {
    let words = ["zocd","gjkl","hzqk","hzgq","gjkl"].iter().map(|&s|s.into()).collect();
    println!("{:?}", unique_morse_representations(words));
}
