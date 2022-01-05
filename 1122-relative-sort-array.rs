pub fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
    use std::collections::HashMap;
    let mut mmp = HashMap::new();
    for num in arr1 {
        if arr2.contains(&num) {
            mmp.entry(num).or_insert(Vec::new()).push(num);
        } else {
            mmp.entry(-1).or_insert(Vec::new()).push(num);
        }
    }
    let mut res: Vec<i32> = Vec::new();
    for num in arr2 {
        let mut tmp = mmp[&num].clone();
        res.append(&mut tmp);
    }

    if mmp.get(&-1) != None {
        let mut tmp = mmp[&-1].clone();
        tmp.sort();
        res.append(&mut tmp);
    }
    res
}

fn main() {
    let arr1 = vec![943,790,427,722,860,550,225,846,715,320];
    let arr2 = vec![943,715,427,790,860,722,225,320,846,550];
    println!("{:?}", relative_sort_array(arr1, arr2));
}
