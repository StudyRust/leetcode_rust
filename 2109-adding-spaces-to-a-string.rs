pub fn add_spaces(s: String, spaces: Vec<i32>) -> String {
    let mut s = s;
    let mut z = 0;
    for space in spaces {
        let idx = space + z;
        s.insert(idx as usize, ' ');
        z += 1;
    }
    s
}

fn main() {
    let s = "LeetcodeHelpsMeLearn".to_string();
    let spaces = vec![8,13,15];
    println!("{:?}", add_spaces(s, spaces));
}
