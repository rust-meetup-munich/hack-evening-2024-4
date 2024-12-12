use csv::ReaderBuilder;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::env;

#[derive(Debug, Deserialize)]
struct Record {
    station: String,
    temperature: f64,
}

fn main() {
    let file = env::args().nth(1).expect("Input file must be specified!");

    let mut csv_reader = ReaderBuilder::new()
        .delimiter(b';')
        .has_headers(false)
        .from_path(file)
        .unwrap();
    let results: Vec<Record> = csv_reader.deserialize().map(|r| r.unwrap()).collect();

    let grouped_results: BTreeMap<String, Vec<f64>> =
        results.into_iter().fold(BTreeMap::new(), |mut acc, a| {
            acc.entry(a.station.clone())
                .or_default()
                .push(a.temperature);
            acc
        });

    let mut final_data: Vec<(String, f64, f64, f64)> = grouped_results
        .iter()
        .map(|(k, v)| {
            (
                k.clone(),
                v.iter()
                    .cloned()
                    .min_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap(),
                v.iter().cloned().sum::<f64>() / v.len() as f64,
                v.iter()
                    .cloned()
                    .max_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap(),
            )
        })
        .collect();
    final_data.sort_by(|a, b| a.0.cmp(&b.0));

    let mut i = 1;
    let len = final_data.len();
    println!("{{");
    for data_point in final_data {
        let (station, min, mean, max) = data_point;
        print!("    {station}={min:.1}/{mean:.1}/{max:.1}");
        if i < len {
            println!(",");
        } else {
            println!();
        }
        i += 1;
    }
    println!("}}");
}
