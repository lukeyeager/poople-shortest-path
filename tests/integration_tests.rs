use poople_shortest_path::WordSet;

#[test]
fn test_list_words_at_distance_zero() {
    let word_set = WordSet::load();
    let result = word_set.list_words_at_distance("POOP", 0);
    assert_eq!(result, vec!["POOP"]);
}

#[test]
fn test_list_words_at_distance_one() {
    let word_set = WordSet::load();
    let result = word_set.list_words_at_distance("POOP", 1);
    assert!(result.contains(&"COOP".to_string()));
    assert!(result.contains(&"POOL".to_string()));
}
