pub fn evaluate(s: String, knowledge: Vec<Vec<String>>) -> String {
    use std::collections::HashMap;
    let mut mmp = HashMap::new();
    for pair in knowledge {
        mmp.insert(pair[0].clone(), pair[1].clone());
    }
    // println!("{:?}", mmp);
    let mut ret = String::new();
    let mut current_word = String::new();
    let mut start = false;
    for c in s.chars() {
        if c == '('  {
            start = true;
            continue
        }
        if c == ')'  {
            start = false;
            let get = mmp.get(&current_word);
            if get == None {
                ret.push('?')
            } else {
                ret.push_str(get.unwrap())
            }
            current_word = String::new();
            continue
        }
        if start {
            current_word.push(c)
        } else {
            ret.push(c)
        }
    }
    ret
}

fn main() {
    println!(
        "{:?}", 
        evaluate(
            "(name)is(age)yearsold(asd)".to_string(), 
            vec![
                vec!["name".to_string(), "bob".to_string()],
                vec!["age".to_string(), "two".to_string()]
            ]
        )
    );
}
