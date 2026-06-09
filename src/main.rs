use clap::{Command, Arg};
use poople_shortest_path::WordSet;

fn main() {
    let app = Command::new("poople")
        .about("Word path solver")
        .subcommand(
            Command::new("list-words-at-distance")
                .about("List all words at a given distance from POOP")
                .arg(
                    Arg::new("distance")
                        .value_name("DISTANCE")
                        .required(true)
                        .help("Distance from POOP"),
                ),
        );

    let matches = app.get_matches();

    if let Some(("list-words-at-distance", sub_matches)) = matches.subcommand() {
        let distance: usize = sub_matches
            .get_one::<String>("distance")
            .expect("Distance is required")
            .parse()
            .expect("Distance must be a number");

        let word_set = WordSet::load();
        let mut results = word_set.list_words_at_distance("POOP", distance);
        results.sort();

        for word in results {
            println!("{}", word);
        }
    }
}
