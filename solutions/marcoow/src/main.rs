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
    for result in csv_reader.deserialize() {
        let record: Record = result.unwrap();
        println!("{:?}", record);
    }
}
