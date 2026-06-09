use std::fs;

fn main() {
    let contents = fs::read_to_string("/usr/share/dict/american-english")
        .expect("Failed to read dictionary file");

    let count = contents
        .lines()
        .filter(|word| word.chars().count() == 4)
        .count();

    println!("{} words", count);
}
