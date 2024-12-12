use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;
use clap::Parser;
use anyhow::{anyhow, Result};
use crate::command::Args;
use crate::weather::Weather;

mod command;
mod weather;

fn main() {
    let args = Args::parse();

    println!("Input: {}", args.filename);

    let map = match read_weather(&args.filename) {
        Ok(map) => map,
        Err(e) => {
            eprintln!("Error: {}", e.to_string());
            exit(-1)
        }
    };

    let mut keys = map.keys().collect::<Vec<_>>();

    keys.sort();

    println!("{}", '{');
    for key in keys {
        let item = map.get(key).unwrap();
        let summary = item.summarize(key);

        println!("    {},", summary);
    }
    print!("{}", '}');
}

fn read_weather(filename: &str) -> Result<HashMap<String,Weather>> {
    let mut map = HashMap::<String,Weather>::new();
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    for line in reader.lines().flatten() {
        let parts = line.split(";").collect::<Vec<_>>();

        if parts.len() != 2 {
            return Err(anyhow!("Invalid line format {}", line));
        }

        let (name,value) = (parts[0], parts[1].parse::<f64>()?);

        let mut item = match map.get(name) {
            Some(item) => item.clone(),
            None => Weather::new()
        };

        item.add(value);
        map.insert(name.to_string(), item);
    }

    Ok(map)
}
