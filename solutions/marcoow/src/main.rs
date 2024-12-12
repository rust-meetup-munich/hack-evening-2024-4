use csv::ReaderBuilder;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::env;

#[derive(Debug, Deserialize)]
struct Record {
    station: String,
    temperature: f32,
}

#[derive(Debug)]
struct FinalRecord {
    station: String,
    min: f32,
    mean: f32,
    max: f32,
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

    let mut final_data: Vec<FinalRecord> = grouped_results
        .iter()
        .map(|(k, v)| FinalRecord {
            station: k.clone(),
            min: v
                .iter()
                .cloned()
                .min_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap(),
            mean: v.iter().cloned().sum::<f32>() / v.len() as f32,
            max: v
                .iter()
                .cloned()
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap(),
        })
        .collect();
    final_data.sort_by(|a, b| a.station.cmp(&b.station));

    println!("{final_data:?}");
}
