// 示例 1：

// 输入：firstWord = "acb", secondWord = "cba", targetWord = "cdb"
// 输出：true
// 解释：
// firstWord 的数值为 "acb" -> "021" -> 21
// secondWord 的数值为 "cba" -> "210" -> 210
// targetWord 的数值为 "cdb" -> "231" -> 231
// 由于 21 + 210 == 231 ，返回 true

// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/check-if-word-equals-summation-of-two-words
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

pub fn is_sum_equal(first_word: String, second_word: String, target_word: String) -> bool {
    fn str_number(s: String) -> i32 {
        let char_vec: Vec<String> = s.chars().map(|e|(e as i32 - 'a' as i32).to_string()).collect();
        char_vec.join("").parse().unwrap()
    }
    str_number(first_word) + str_number(second_word) == str_number(target_word)
}

fn main() {
    println!("{:?}", is_sum_equal("acb".to_string(), "cba".to_string(), "cdb".to_string()));
}
