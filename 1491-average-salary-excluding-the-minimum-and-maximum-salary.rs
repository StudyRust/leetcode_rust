pub fn average(salary: Vec<i32>) -> f64 {
    let mut sorted_salary = salary.clone();
    sorted_salary.sort();
    let length = salary.len() - 1;
    sorted_salary[1..length].iter().sum::<i32>() as f64 / (length - 1) as f64
}

fn main() {
    let salary = vec![8000,9000,2000,3000,6000,1000];
    println!("{:?}", average(salary));
}
