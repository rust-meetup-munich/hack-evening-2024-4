use std::io::{BufRead, BufReader};
use std::fs::{File, OpenOptions};

use clap::Parser;
use memmap::MmapOptions;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Input file.
    input_file: String,
}

struct Stats {
    mean: f64,
    min: f64,
    max: f64,
    count: i32,
}

fn main() {
    // Read the input file using clap
    let args = Cli::parse();
    let input_file = args.input_file;
    solution_bulk(input_file);
}

fn solution_dummy(input_file: String) {
    // Create the solution hashmap
    let mut solution: std::collections::BTreeMap<String, Stats> = std::collections::BTreeMap::new();

    // Read the input file line by line
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);
    // There should be an opportunity for memory mapping the file here
    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split(";").collect();
        let key_str = parts[0];
        let value_str = parts[1]; // as float
        let v = value_str.trim().parse::<f64>().unwrap();
        if solution.contains_key(&key_str.to_owned()) {
            // Update the entry
            let stats = solution.get_mut(key_str).unwrap();
            stats.count += 1;
            stats.mean = (stats.mean * (stats.count-1) as f64 + v) / stats.count as f64;
            stats.min = stats.min.min(v);
            stats.max = stats.max.max(v);
        } else {
            // Create a new entry
            solution.insert(key_str.to_owned(), Stats{mean: v, min: v, max: v, count: 1});
        }
    }

    // Print the solution
    println!("{{\n");
    for (key, stats) in solution.iter() {
        let line = format!("\t{}={:.1}/{:.1}/{:.1},", key, stats.min, stats.mean, stats.max);
        println!("{}", line);
    }
    println!("}}");
}

fn solution_bulk(input_file: String) {
    // Create the solution hashmap
    let mut solution: std::collections::BTreeMap<String, Vec<f64>> = std::collections::BTreeMap::new();

    // Read the input file line by line
    let file = OpenOptions::new().read(true).open(input_file).unwrap();
    // There should be an opportunity for memory mapping the file here
    // let mmap = unsafe { MmapOptions::new().map(&file).unwrap() };
    // let reader = BufReader::new(mmap.as_ref());
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split(";").collect();
        let key_str = parts[0];
        let value_str = parts[1]; // as float
        let v = value_str.trim().parse::<f64>().unwrap();
        solution.entry(key_str.to_owned()).or_insert(Vec::new()).push(v);
    }

    // Print the solution
    println!("{{\n");
    for (key, stats) in solution.iter_mut() {
        stats.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let length = stats.len();
        // Calculate the mean
        let sum: f64 = stats.iter().sum();
        let mean = sum / length as f64;
        let line = format!("\t{}={:.1}/{:.1}/{:.1},", key, stats[0], mean, stats[length-1]);
        println!("{}", line);
    }
    println!("}}");
}
