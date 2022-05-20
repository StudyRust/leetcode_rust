pub fn convert_time(current: String, correct: String) -> i32 {
    pub fn time_to_mins(time: String) -> i32 {
        let sp: Vec<_> = time.split(":").collect();
        sp[0].parse::<i32>().unwrap() * 60 + sp[1].parse::<i32>().unwrap()
    }
    let mut minus = time_to_mins(correct) - time_to_mins(current);
    let mut ret = 0;
    while minus > 0 {
        let min_60 = minus / 60;
        if min_60 != 0 {
            ret += min_60;
            minus -= min_60 * 60;
            continue;
        }
        let min_15 = minus / 15;
        if min_15 != 0 {
            ret += min_15;
            minus -= min_15 * 15;
            continue;
        }
        let min_5 = minus / 5;
        if min_5 != 0 {
            ret += min_5;
            minus -= min_5 * 5;
            continue;
        }
        let min_1 = minus / 1;
        if min_1 != 0 {
            ret += min_1;
            minus -= min_1 * 1;
            continue;
        }
    }
    ret
}

fn main() {
    let current = "11:00".to_string();
    let correct = "11:01".to_string();
    println!("{:?}", convert_time(current, correct));
}
