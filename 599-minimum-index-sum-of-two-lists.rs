pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
    let mut ret_idx = vec![2000; list1.len()];
    let mut ret = vec!();
    for (i, s) in list1.iter().enumerate() {
        let tmp = list2.iter().position(|x|x == s);
        if tmp.is_some() {
            ret_idx[i] = tmp.unwrap()+i;
        }
    }
    let min = ret_idx.iter().min().unwrap();
    for (i, idx) in ret_idx.iter().enumerate() {
        if idx == min {
            ret.push(list1[i].clone());
        }
    }
    ret
}

fn main() {
    let list2 = vec!["vacag","KFC"]
        .iter().map(|e|e.to_string()).collect::<Vec<String>>();
    let list1 = vec!["fvo","xrljq","jrl","KFC"]
        .iter().map(|e|e.to_string()).collect::<Vec<String>>();
    println!("{:?}", find_restaurant(list1, list2));
}
