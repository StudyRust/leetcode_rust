// 示例 1：

// 输入：s = "ab", goal = "ba"
// 输出：true
// 解释：你可以交换 s[0] = 'a' 和 s[1] = 'b' 生成 "ba"，此时 s 和 goal 相等。

// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/buddy-strings
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
// 官方题解, 其实情况没有那么多, 就三种情况:

// 1 字符串长度不相等, 直接返回false
// 2 字符串相等的时候, 只要有重复的元素就返回true
// 3 A, B字符串有不相等的两个地方, 需要查看它们交换后是否相等即可.



pub fn buddy_strings(s: String, goal: String) -> bool {
    if &s.len() != &goal.len() {
        return false;
    }
    if s == goal {
        let mut s_chars: Vec<char> = s.clone().chars().collect();
        s_chars.sort_unstable();
        s_chars.dedup();
        s_chars.len() < s.len()
    } else {
        let mut tmp: Vec<char> = vec!();
        for (i, c) in s.chars().enumerate() {
            let o = goal.chars().nth(i).unwrap();
            if c != o {
                tmp.push(c);
                tmp.push(o);
            }
            if tmp.len() > 4 {
                return false;
            }
        }
        if tmp.len() != 4 {
            return false;
        }
        tmp[0] == tmp[3] && tmp[1] == tmp[2]
    }
}

fn main() {
    let s = "abab".to_string();
    let goal = "abab".to_string();
    println!("{:?}", buddy_strings(s, goal));
}
