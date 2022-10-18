pub fn num_smaller_by_frequency(queries: Vec<String>, words: Vec<String>) -> Vec<i32> {
    pub fn string_to_value(s: String) -> i32 {
        let mut cs = s.chars().collect::<Vec<char>>();
        let min = cs.clone().into_iter().min().unwrap();
        cs.retain(|c|*c == min);
        cs.len() as i32
    }
    let mut ret = vec![];
    for q in queries {
        let q_value = string_to_value(q);
        let mut tmp = 0;
        for w in &words {
            if string_to_value(w.clone()) > q_value { tmp += 1 }
        }
        ret.push(tmp)
    }
    ret
}

fn main() {
    let queries = vec!["cbd".to_string()];
    let words = vec!["zaaaz".to_string()];
    println!("{:?}", num_smaller_by_frequency(queries, words));
    let queries = vec!["bbb".to_string(),"cc".to_string()];
    let words = vec!["a".to_string(),"aa".to_string(),"aaa".to_string(),"aaaa".to_string()];
    println!("{:?}", num_smaller_by_frequency(queries, words));
}
