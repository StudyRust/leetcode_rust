
pub fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
    let mut ret = vec!();
    let mut last = 0;
    for i in target {
        if i-last == 1 {
            ret.push("Push".to_string());
        } else {
            for _ in 0..(i-last) {
                ret.push("Push".to_string());
                ret.push("Pop".to_string());
            }
            ret.remove(ret.len()-1);
        }
        last = i;
    }
    ret
}

fn main() {
    let target = vec![1,2];
    let n = 4;
    println!("{:?}", build_array(target, n));
}
