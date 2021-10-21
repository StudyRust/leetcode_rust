fn main() {
    let paths = vec!(
        vec!("London".to_string(), "New York".to_string()),
        vec!("New York".to_string(), "Lima".to_string()),
        vec!("Lima".to_string(), "Sao Paulo".to_string())
    );
    println!("{:?}", dest_city(paths));
}

pub fn dest_city(paths: Vec<Vec<String>>) -> String {
    let mut res = vec!();
    for path in paths {
        for (i, des) in path.into_iter().enumerate() {
            if res.contains(&des) {
                res.retain(|value| *value != des);
            } else {
                if i == 0 {
                    res.insert(0, des);
                } else {
                    res.push(des);
                }
            }
        }
    }
    res[1].to_string()
}
