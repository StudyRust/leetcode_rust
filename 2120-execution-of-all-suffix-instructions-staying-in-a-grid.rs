pub fn execute_instructions(n: i32, start_pos: Vec<i32>, s: String) -> Vec<i32> {
    let mut res: Vec<i32> = vec!();
    for i in 0..s.len() {
        let (_, last) = s.split_at(i);
        let commands: Vec<char> = last.chars().collect();
        let mut step = 0;
        let mut x = start_pos[1];
        let mut y = start_pos[0];
        for command in commands {
            if command == 'U' {
                y -= 1;
            } else if command == 'D' {
                y += 1;
            } else if command == 'L' {
                x -= 1;
            } else {
                x += 1;
            }
            if x < 0 || y < 0 || x > n - 1 || y > n - 1 {
                break;
            }
            step += 1;
        }
        res.push(step);
    }
    res
}

fn main() {
    let n = 3;
    let start_pos = vec![0,1];
    let s = "RRDDLU".to_string();
    println!("{:?}", execute_instructions(n, start_pos, s));
}
