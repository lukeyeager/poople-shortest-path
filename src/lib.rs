use std::collections::HashSet;
use std::fs;

pub struct WordSet {
    pub words: HashSet<String>,
}

impl WordSet {
    pub fn load() -> Self {
        let contents = fs::read_to_string("/usr/share/dict/american-english")
            .expect("Failed to read dictionary file");

        let words = contents
            .lines()
            .filter(|word| {
                word.len() == 4 && word.chars().all(|c| c.is_ascii_alphabetic())
            })
            .map(|word| word.to_uppercase())
            .collect::<HashSet<_>>();

        WordSet { words }
    }

    pub fn list_words_at_distance(&self, word: &str, distance: usize) -> Vec<String> {
        self.words
            .iter()
            .filter(|w| character_distance(word, w) == distance)
            .cloned()
            .collect()
    }
}

pub fn character_distance(word1: &str, word2: &str) -> usize {
    word1
        .chars()
        .zip(word2.chars())
        .filter(|(c1, c2)| c1 != c2)
        .count()
}
