// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/count-items-matching-a-rule
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

fn main() {
    let items = vec!(
        vec!("phone","blue","pixel"),
        vec!("computer","silver","lenovo"),
        vec!("phone","gold","iphone")).into_iter().map(|a|
            a.into_iter().map(|b|
                b.to_string()
            ).collect()
        ).collect();
    let rule_key = "color".to_string();
    let rule_value = "silver".to_string();
    let res = count_matches(items, rule_key, rule_value);
    println!("{:?}", res);
}

pub fn count_matches(items: Vec<Vec<String>>, rule_key: String, rule_value: String) -> i32 {
    let mut res = 0;
    let mut x = 0;
    if rule_key == "type" {
        x = 0;
    } else if rule_key == "color" {
        x = 1;
    } else if rule_key == "name" {
        x = 2;
    }
    for item in items {
        if item[x] == rule_value {
            res += 1;
        }
    }
    res
}
