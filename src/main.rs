use std::time::Instant;
use clap::{Command, Arg};
use poople_shortest_path::WordSet;

fn main() {
    env_logger::init();

    let app = Command::new("poople")
        .about("Word path solver")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("list-words-at-distance")
                .about("List all words reachable from POOP in exactly N steps on the word graph")
                .arg(
                    Arg::new("distance")
                        .value_name("DISTANCE")
                        .required(true)
                        .help("Graph distance from POOP"),
                ),
        );

    let matches = app.get_matches();

    if let Some(("list-words-at-distance", sub_matches)) = matches.subcommand() {
        let distance: usize = sub_matches
            .get_one::<String>("distance")
            .expect("Distance is required")
            .parse()
            .expect("Distance must be a number");

        let t = Instant::now();
        let word_set = WordSet::load();
        log::debug!("graph build: {:.2?}", t.elapsed());

        let t2 = Instant::now();
        let mut results = word_set.list_words_at_distance("POOP", distance);
        log::debug!("bfs:         {:.2?}", t2.elapsed());
        log::debug!("total:       {:.2?}", t.elapsed());

        if results.is_empty() {
            eprintln!("No words found at distance {distance}");
            std::process::exit(1);
        }

        results.sort();
        for word in &results {
            println!("{word}");
        }
        log::info!("{} word(s) at distance {distance}", results.len());
    }
}
