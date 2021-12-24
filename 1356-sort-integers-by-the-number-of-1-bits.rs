pub fn sort_by_bits(arr: Vec<i32>) -> Vec<i32> {
    use std::collections::HashMap;
    let mut count_map: HashMap<usize, Vec<i32>> = HashMap::new();
    for num in arr {
        let num_binary: Vec<char> = format!("{:b}", num).chars().collect();
        let count_binary = num_binary.iter().filter(|&n|*n=='1').count();
        count_map.entry(count_binary).or_insert(Vec::new()).push(num);
    }
    let mut count_vec: Vec<_> = count_map.iter().collect();
    count_vec.sort_by(|a, b| a.0.cmp(b.0));
    let mut res: Vec<i32> = vec!();
    for e in count_vec {
        let mut sorted_arr = e.1.clone();
        sorted_arr.sort();
        for i in sorted_arr {
            res.push(i);
        }
    }
    res
}

fn main() {
    println!("{:?}", sort_by_bits(vec![0,1,2,3,4,5,6,7,8]));
    println!("{:?}", sort_by_bits(vec![1024,512,256,128,64,32,16,8,4,2,1]));
    println!("{:?}", sort_by_bits(vec![10,100,1000,10000]));
}
