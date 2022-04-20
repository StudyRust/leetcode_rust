pub fn int_to_roman(num: i32) -> String {
    let mut num = num;
    let values = vec![1000,900,500,400,100,90,50,40,10,9,5,4,1];
    let reps = vec!["M","CM","D","CD","C","XC","L","XL","X","IX","V","IV","I"];
    let mut ret = String::new();
    for i in 0..13 {
        while num >= values[i] {
            num -= values[i];
            ret.push_str(reps[i]);
        }
    }
    ret.to_string()
}

fn main() {
    let num = 1994;
    println!("{:?}", int_to_roman(num));
}
