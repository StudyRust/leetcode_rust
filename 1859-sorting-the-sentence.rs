// 示例 1：

// 输入：s = "is2 sentence4 This1 a3"
// 输出："This is a sentence"
// 解释：将 s 中的单词按照初始位置排序，得到 "This1 is2 a3 sentence4" ，然后删除数字。

// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/sorting-the-sentence
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

fn main() {
    println!("{:?}", sort_sentence("is2 sentence4 This1 a3".to_string()));
}

pub fn sort_sentence(s: String) -> String {
    let mut arr = s.split(' ').collect::<Vec<&str>>();
    arr.sort_by(|b, a| b.chars().last().unwrap().cmp(&a.chars().last().unwrap())); // sort 方法必须换行，不能链式调用
    let res = arr.join(" ");
    res.chars().filter(|c|!c.is_digit(10)).collect::<String>()
}
