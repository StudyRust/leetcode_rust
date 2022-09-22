pub fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
    let mut current_nil_bottles = 0;
    let mut num_bottles = num_bottles;
    let mut ret = 0;
    loop {
        ret += num_bottles;
        current_nil_bottles += num_bottles;
        num_bottles = 0;
        let tmp = current_nil_bottles / num_exchange;
        if tmp > 0 {
            num_bottles += tmp;
            current_nil_bottles -= tmp * num_exchange;
        }
        if num_bottles == 0 && tmp == 0 { break }
    }
    ret
}

fn main() {
    println!("{:?}", num_water_bottles(15, 4));
}
