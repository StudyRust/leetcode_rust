// 抄答案啦
// 可以理解为，将level=1 以上的括号写入结果，level 指的是深度
// 妙啊

pub fn remove_outer_parentheses(s: String) -> String {
    let mut level = 0;
    let mut ret = String::new();
    for c in s.chars() {
        if c == ')' {
            level -= 1;
        }
        if level >= 1 {
            ret.push(c);
        }
        if c == '(' {
            level += 1;
        }
    }
    ret
}

fn main() {
    let s = "(()())(())".to_string();
    println!("{:?}", remove_outer_parentheses(s));
}
