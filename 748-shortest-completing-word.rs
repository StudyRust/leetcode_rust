pub fn shortest_completing_word(license_plate: String, words: Vec<String>) -> String {
    use std::collections::HashMap;
    let mut counter_map = HashMap::new();
    let mut str_map = HashMap::new();

    for c in license_plate.chars() {
        if c.is_alphabetic() {
            let coun = str_map.entry(c.to_lowercase().to_string()).or_insert(0);
            *coun += 1;
        }
    }

    'outer: for word in words {
        let mut word_map = HashMap::new();
        for c in word.chars() {
            let coun = word_map.entry(c.to_string()).or_insert(0);
            *coun += 1;
        }
        for (k, v) in &str_map {
            if word_map.get(k) == None || word_map[k] != *v {
                continue 'outer;
            }
        }
        counter_map.entry(word.len()).or_insert(Vec::new()).push(word);
    }

    let mut counter_vec: Vec<_> = counter_map.iter().collect();
    counter_vec.sort_by(|a, b|a.0.cmp(b.0));
    println!("{:?}", counter_map);
    if counter_vec.iter().count() == 0 {
        String::new()
    } else {
        counter_vec[0].1[0].to_string()
    }
}

fn main() {
    let license_plate = "GrC8950".to_string();
    let words: Vec<String> = vec!["measure","other","every","base","according","level","meeting","none","marriage","rest"].iter().map(|e|e.to_string()).collect();
    println!("{:?}", shortest_completing_word(license_plate, words));
}
