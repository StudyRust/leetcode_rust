pub fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
    let mut tickets = tickets;
    let mut ret = 0;
    while tickets[k as usize] != 0 {
        for i in 0..tickets.len() {
            if tickets[i] != 0 {
                tickets[i] -= 1;
                ret += 1;
            }
            if tickets[k as usize] == 0 {
                return ret
            }
        }
    }
    ret
}

fn main() {
    println!("{:?}", time_required_to_buy(vec![2,3,2], 2));
    println!("{:?}", time_required_to_buy(vec![84,49,5,24,70,77,87,8], 3));
}
