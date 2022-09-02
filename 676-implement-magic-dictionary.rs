use std::collections::HashMap;

struct MagicDictionary {
    mmp: HashMap<usize, Vec<Vec<char>>>
}

impl MagicDictionary {
    fn new() -> Self {
        MagicDictionary {
            mmp: HashMap::new()
        }
    }

    fn build_dict(&mut self, dictionary: Vec<String>) {
        for s in dictionary {
            self.mmp.entry(s.len()).or_insert(Vec::new()).push(s.chars().collect::<Vec<char>>());
        }
    }

    fn search(&self, search_word: String) -> bool {
        let search_word_len = search_word.len();
        let search_res = self.mmp.get(&search_word_len);
        let search_chars = search_word.chars().collect::<Vec<char>>();
        if search_res == None {
            false
        } else {
            'outer: for chars in search_res.unwrap() {
                let mut count = 0;
                for i in 0..search_word_len {
                    if chars[i] != search_chars[i] { count += 1 }
                    if count > 1 { continue 'outer }
                }
                if count == 1 { return true }
            }
            false
        }
    }
}

fn main() {
    let mut obj = MagicDictionary::new();
    obj.build_dict(vec!["hello".to_string(),"hallo".to_string(), "judge".to_string()]);
    let ret_2: bool = obj.search("judga".to_string());
    println!("{:?}", ret_2);
}
