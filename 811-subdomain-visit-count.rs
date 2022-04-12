pub fn subdomain_visits(cpdomains: Vec<String>) -> Vec<String> {
    use std::collections::HashMap;
    let mut mmp: HashMap<String, usize> = HashMap::new();

    for c in cpdomains {
        let arr: Vec<String> = c.split(" ").map(|e|e.to_string()).collect();
        let num: usize = arr[0].parse().unwrap();
        let url_arr: Vec<String> = arr[1].split(".").map(|e|e.to_string()).collect();
        if url_arr.iter().count() == 3 {
            let three = url_arr.join(".");
            let count = mmp.entry(three.to_string()).or_insert(0);
            *count += num;
            let two = url_arr[1..].join(".");
            let count = mmp.entry(two.to_string()).or_insert(0);
            *count += num;
            let one = &url_arr[2];
            let count = mmp.entry(one.to_string()).or_insert(0);
            *count += num;
        }
        if url_arr.iter().count() == 2 {
            let two = url_arr.join(".");
            let count = mmp.entry(two.to_string()).or_insert(0);
            *count += num;
            let one = &url_arr[1];
            let count = mmp.entry(one.to_string()).or_insert(0);
            *count += num;
        }
        if url_arr.iter().count() == 1 {
            let one = &url_arr[0];
            let count = mmp.entry(one.to_string()).or_insert(0);
            *count += num;
        }
    }
    let mut ret: Vec<String> = vec!();
    for (k, v) in mmp {
        let mut tmp = String::new();
        tmp.push_str(&v.to_string());
        tmp.push(' ');
        tmp.push_str(&k);
        ret.push(tmp);
    }
    ret
}

fn main() {
    let cpdomains = vec!["900 google.mail.com", "50 yahoo.com", "1 intel.mail.com", "5 wiki.org"]
        .iter().map(|e|e.to_string()).collect();
    println!("{:?}", subdomain_visits(cpdomains));
}
