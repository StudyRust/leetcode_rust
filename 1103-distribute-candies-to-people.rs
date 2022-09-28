pub fn distribute_candies(candies: i32, num_people: i32) -> Vec<i32> {
    let mut ret = vec![0; num_people as usize];
    let mut candies = candies;
    let mut tmp = 1;
    while candies > 0 {
        for i in 0..ret.len() {
            if candies >= tmp {
                ret[i] += tmp;
                candies -= tmp;
            } else {
                ret[i] += candies;
                candies = 0;
            }
            tmp += 1;
        }
    }
    ret
}

fn main() {
    println!("{:?}", distribute_candies(7, 4));
}
