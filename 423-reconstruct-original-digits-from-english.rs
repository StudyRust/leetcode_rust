// 脑筋急转弯
// 数学归纳法
pub fn original_digits(s: String) -> String {
    let string = s;
    let mut z = 0;
    let mut w = 0;
    let mut g = 0;
    let mut x = 0;
    let mut t = 0;
    let mut r = 0;
    let mut s = 0;
    let mut o = 0;
    let mut v = 0;
    let mut i = 0;
    for c in string.chars().collect::<Vec<char>>() {
        match c {
            'z' => z += 1,
            'w' => w += 1,
            'g' => g += 1,
            'x' => x += 1,
            't' => t += 1,
            'r' => r += 1,
            's' => s += 1,
            'o' => o += 1,
            'v' => v += 1,
            'i' => i += 1,
            _ => {},
        }
    }
    let mut n = vec!(0; 10);
    n[0] = z;
    n[2] = w;
    n[8] = g;
    n[6] = x;
    n[3] = t - n[2] - n[8];
    n[4] = r - n[3] - n[0];
    n[7] = s - n[6];
    n[1] = o - n[4] - n[2] - n[0];
    n[5] = v - n[7];
    n[9] = i - n[8] - n[6] - n[5];
    let mut ret = String::new();
    for (i, e) in n.iter().enumerate() {
        if *e == 0 { continue }
        for _ in 0..*e {
            ret.push_str(&i.to_string());
        }
    }
    ret
}

fn main() {
    let s = "owoztneoer".to_string();
    println!("{:?}", original_digits(s));
}
