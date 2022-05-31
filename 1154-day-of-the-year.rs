pub fn day_of_year(date: String) -> i32 {
    let run_year = vec!(
        31, 29, 31, 30,
        31, 30, 31, 30,
        31, 30, 31, 30
    );
    let un_run_year = vec!(
        31, 28, 31, 30,
        31, 30, 31, 30,
        31, 30, 31, 30
    );
    fn is_run_year(year: String) -> bool {
        let year: i32 = year.parse().unwrap();
        (year % 100 == 0 && year % 40 == 0) ||
        (year % 100 != 0 && year % 4 == 0)
    }
    let arr: Vec<String> = date.split("-")
        .map(|e|e.to_string())
        .collect();
    let mut month = arr[1].parse().unwrap();
    let day: i32 = arr[2].parse().unwrap();
    let year = arr[0].clone();
    if month == 1 {
        day
    } else {
        month -= 1;
        if is_run_year(year) {
            run_year[0..month].iter().sum::<i32>() + day
        } else {
            un_run_year[0..month].iter().sum::<i32>() + day
        }
    }
}

fn main() {
    let date = "2004-03-01".to_string();
    println!("{:?}", day_of_year(date));
}
