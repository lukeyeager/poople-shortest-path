use std::fs;

fn main() {
    let contents = fs::read_to_string("/usr/share/dict/american-english")
        .expect("Failed to read dictionary file");

    let count = contents
        .lines()
        .filter(|word| {
            word.len() == 4 && word.chars().all(|c| c.is_ascii_alphabetic())
        })
        .count();

    println!("{} words", count);
}
