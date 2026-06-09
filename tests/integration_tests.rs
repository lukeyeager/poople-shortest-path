use poople_shortest_path::WordSet;

#[test]
fn test_list_zero() {
    let word_set = WordSet::load();
    let result = word_set.list_words_at_distance("POOP", 0);
    assert_eq!(result, vec!["POOP"]);
}

#[test]
fn test_list_one() {
    let word_set = WordSet::load();
    let result = word_set.list_words_at_distance("POOP", 1);
    assert!(result.contains(&"COOP".to_string()));
    assert!(result.contains(&"POOL".to_string()));
}

#[test]
fn play_poop_length_0() {
    let ws = WordSet::load();
    let chain = ws.play("POOP");
    assert_eq!(chain, vec!["POOP"]);
}

#[test]
fn play_pool_to_poop() {
    let ws = WordSet::load();
    let chain = ws.play("POOL");
    assert_eq!(chain, vec!["POOL", "POOP"]);
}

#[test]
fn play_edgy_length_15() {
    let ws = WordSet::load();
    let chain = ws.play("EDGY");
    assert_eq!(chain.len() - 1, 15);
}
