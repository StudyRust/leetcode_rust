pub fn judge_circle(moves: String) -> bool {
    let (mut x, mut y) = (0, 0);
    for c in moves.chars() {
        match c {
            'U' => x += 1,
            'D' => x -= 1,
            'L' => y += 1,
            'R' => y -= 1,
            _ => {},
        }
    }
    x == 0 && y == 0
}

fn main() {
    let moves = "UD".to_string();
    println!("{:?}", judge_circle(moves));
}
