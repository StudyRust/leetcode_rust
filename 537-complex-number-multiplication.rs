pub fn complex_number_multiply(num1: String, num2: String) -> String {
    let a = num1.split("+").collect::<Vec<&str>>();
    let x1: i32 = a[0].parse().unwrap();
    let y1: i32 = a[1].replace("i", "").parse().unwrap();
    let b = num2.split("+").collect::<Vec<&str>>();
    let x2: i32 = b[0].parse().unwrap();
    let y2: i32 = b[1].replace("i", "").parse().unwrap();
    // println!("{:?} {:?} {:?} {:?}", x1, y1, x2, y2);
    let real = x1 * x2 - y1 * y2;
    let blural = x1 * y2 + y1 * x2;
    format!("{:?}+{:?}i", real, blural)
}

fn main() {
    let num1 = "1+-1i".to_string();
    let num2 = "1+-1i".to_string();
    println!("{:?}", complex_number_multiply(num1, num2));
}
