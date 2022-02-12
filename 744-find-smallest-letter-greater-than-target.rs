pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
    for letter in &letters {
        if letter > &target {
            return *letter;
        }
    }
    *letters.iter().nth(0).unwrap()
}

fn main() {
    let letters = vec!['c', 'f', 'j'];
    let target = 'j';
    println!("{:?}", next_greatest_letter(letters, target));
}
