// 示例 1：

// 输入：command = "G()(al)"
// 输出："Goal"
// 解释：Goal 解析器解释命令的步骤如下所示：
// G -> G
// () -> o
// (al) -> al
// 最后连接得到的结果是 "Goal"

// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/goal-parser-interpretation
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

fn main() {
    let s = "(al)G(al)()()G".to_string();
    let res = interpret(s);
    println!("{:?}", res);
}

// 100% 100% haha
pub fn interpret(command: String) -> String {
    command.replace("(al)", "al").replace("()", "o")
}