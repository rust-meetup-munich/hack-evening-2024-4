use csv::ReaderBuilder;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::env;

#[derive(Debug, Deserialize)]
struct Record {
    station: String,
    temperature: f32,
}

fn main() {
    let file = env::args().nth(1).expect("Input file must be specified!");

    let mut csv_reader = ReaderBuilder::new()
        .delimiter(b';')
        .has_headers(false)
        .from_path(file)
        .unwrap();
    let results: Vec<Record> = csv_reader.deserialize().map(|r| r.unwrap()).collect();

    let grouped_results: BTreeMap<String, Vec<f32>> =
        results.into_iter().fold(BTreeMap::new(), |mut acc, a| {
            acc.entry(a.station.clone())
                .or_default()
                .push(a.temperature);
            acc
        });

    println!("{grouped_results:?}");
}
