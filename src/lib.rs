use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

pub struct WordSet {
    pub words: HashSet<String>,
    adjacency: HashMap<String, Vec<String>>,
}

impl WordSet {
    pub fn load() -> Self {
        let contents = fs::read_to_string("/usr/share/dict/american-english")
            .expect("Failed to read dictionary file");

        let words: HashSet<String> = contents
            .lines()
            .filter(|word| word.len() == 4 && word.chars().all(|c| c.is_ascii_alphabetic()))
            .map(|word| word.to_uppercase())
            .collect();

        let adjacency = build_adjacency(&words);
        WordSet { words, adjacency }
    }

    pub fn list_words_at_distance(&self, source: &str, distance: usize) -> Vec<String> {
        bfs(&self.adjacency, source, distance)
    }

    pub fn play(&self, source: &str) -> Vec<String> {
        let t = std::time::Instant::now();
        let path = bfs_path(&self.adjacency, source, "POOP");
        log::debug!("bfs_path:    {:.2?}", t.elapsed());
        path
    }
}

// Builds adjacency list by generating all single-character substitutions per word
// and checking membership — O(V * 104) vs O(V²) for naive pairwise comparison.
fn build_adjacency(words: &HashSet<String>) -> HashMap<String, Vec<String>> {
    let mut adj: HashMap<String, Vec<String>> = words
        .iter()
        .map(|w| (w.clone(), Vec::new()))
        .collect();

    for word in words {
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(word.as_bytes());

        for pos in 0..4 {
            let original = bytes[pos];
            for c in b'A'..=b'Z' {
                if c == original {
                    continue;
                }
                bytes[pos] = c;
                let neighbor = std::str::from_utf8(&bytes).unwrap();
                if words.contains(neighbor) {
                    adj.get_mut(word).unwrap().push(neighbor.to_string());
                }
            }
            bytes[pos] = original;
        }
    }

    adj
}

// BFS from `source` to `target`, returning the shortest path as a Vec or empty if unreachable.
fn bfs_path(adj: &HashMap<String, Vec<String>>, source: &str, target: &str) -> Vec<String> {
    if source == target {
        return vec![source.to_string()];
    }

    let mut parent: HashMap<String, String> = HashMap::new();
    let mut queue: VecDeque<String> = VecDeque::new();

    parent.insert(source.to_string(), source.to_string());
    queue.push_back(source.to_string());

    while let Some(word) = queue.pop_front() {
        for neighbor in adj.get(&word).into_iter().flatten() {
            if parent.contains_key(neighbor) {
                continue;
            }
            parent.insert(neighbor.clone(), word.clone());
            if neighbor == target {
                let mut path = vec![target.to_string()];
                let mut cur = target;
                loop {
                    let p = &parent[cur];
                    if p == cur {
                        break;
                    }
                    path.push(p.clone());
                    cur = p;
                }
                path.reverse();
                return path;
            }
            queue.push_back(neighbor.clone());
        }
    }

    vec![]
}

// BFS from `source` up to `target_depth`. Stops expanding once nodes at
// `target_depth` are reached, so work is proportional to the subgraph visited —
// O(V' + E') where V', E' are nodes/edges within `target_depth` of `source`.
fn bfs(adj: &HashMap<String, Vec<String>>, source: &str, target_depth: usize) -> Vec<String> {
    let mut visited: HashSet<String> = HashSet::new();
    let mut queue: VecDeque<(String, usize)> = VecDeque::new();
    let mut result: Vec<String> = Vec::new();

    visited.insert(source.to_string());
    queue.push_back((source.to_string(), 0));

    while let Some((word, d)) = queue.pop_front() {
        if d == target_depth {
            result.push(word);
        } else {
            for neighbor in adj.get(&word).into_iter().flatten() {
                if !visited.contains(neighbor) {
                    visited.insert(neighbor.clone());
                    queue.push_back((neighbor.clone(), d + 1));
                }
            }
        }
    }

    result
}
