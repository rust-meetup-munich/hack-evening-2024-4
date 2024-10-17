// use ordermap::OrderMap as HashMap;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    assert_eq!(
        env::args().len(),
        2,
        "Usage: {} <input_file.csv>",
        env::args().next().unwrap()
    );

    // Get the file path from command line arguments
    let file_path = &env::args().nth(1).unwrap();

    // Open the file
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    // HashMap to store city data
    let mut city_data: HashMap<String, Vec<f64>> = HashMap::new();

    // Read the file line by line
    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(';').collect();
        if parts.len() != 2 {
            continue; // Skip lines that don't have exactly two parts
        }

        let city = parts[0].to_string();
        let temperature: f64 = match parts[1].parse() {
            Ok(temp) => temp,
            Err(_) => continue, // Skip lines with invalid temperature values
        };

        city_data
            .entry(city)
            .or_insert_with(Vec::new)
            .push(temperature);
    }

    // Calculate min, mean, and max temperatures for each city
    let mut results = String::from("{\n    ");
    let mut seperator = "";
    for (city, temps) in &city_data {
        let min_temp = temps.iter().cloned().fold(f64::INFINITY, f64::min);
        let max_temp = temps.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let mean_temp = temps.iter().sum::<f64>() / temps.len() as f64;

        results.push_str(&format!(
            "{seperator}{city}={min_temp:.1}/{mean_temp:.1}/{max_temp:.1}",
        ));
        seperator = ",\n    ";
    }
    results.push_str("\n}");

    // Print the results
    println!("{results}");

    Ok(())
}
