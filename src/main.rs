use poople_shortest_path::WordSet;

fn main() {
    let word_set = WordSet::load();
    println!("{} words", word_set.words.len());
}
