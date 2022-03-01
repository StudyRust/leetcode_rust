pub fn number_of_beams(bank: Vec<String>) -> i32 {
    let mut count_arr = vec!();
    for e in bank {
        let count = e.matches("1").count();
        if count != 0 {
            count_arr.push(count);
        }
    }
    if count_arr.iter().count() < 2 {
        0
    } else {
        let mut res = 0;
        let index = count_arr.len() - 1;
        for i in 0..index {
            res += count_arr[i] * count_arr[i+1];
        }
        res as i32
    }
}

fn main() {
    let bank: Vec<String> = vec!["011001","000000","010100","001000"]
        .iter().map(|e|e.to_string()).collect();
    println!("{:?}", number_of_beams(bank));
}
