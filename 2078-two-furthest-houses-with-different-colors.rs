pub fn max_distance(colors: Vec<i32>) -> i32 {
    let mut res = 0;
    for i in 0..colors.len() {
        for j in (i+1)..colors.len() {
            let tmp = j - i;
            if colors[i] != colors[j] && tmp > res {
                res = tmp;
            }
        }
    }
    res as i32
}

fn main() {
    let colors = vec![1,1,1,6,1,1,1];
    println!("{:?}", max_distance(colors));
}
