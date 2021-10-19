// 示例 1：

// 输入：s = "(1+(2*3)+((8)/4))+1"
// 输出：3
// 解释：数字 8 在嵌套的 3 层括号中。

// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/maximum-nesting-depth-of-the-parentheses
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

fn main() {
    println!("{:?}", max_depth("(1+(2*3)+((8)/4))+1".to_string()));
}

pub fn max_depth(s: String) -> i32 {
    let s_chars = s.chars();
    let mut max_depth = 0;
    let mut current_depth = 0;
    for c in s_chars {
        if c == '(' {
            current_depth += 1;
        }
        if c == ')' {
            current_depth -= 1;
        }
        if current_depth > max_depth {
            max_depth = current_depth;
        }
    }
    max_depth
}
