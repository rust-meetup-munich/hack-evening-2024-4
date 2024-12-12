use csv::ReaderBuilder;
use serde::Deserialize;
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
    for result in results {
        let record: Record = result;
        println!("{:?}", record);
    }
}
