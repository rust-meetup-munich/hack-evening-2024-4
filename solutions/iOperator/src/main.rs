use clap::Parser;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

use std::collections::BTreeMap;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    filename: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    let filename = cli.filename.unwrap();

    let f = File::open(filename).unwrap();
    let reader = BufReader::new(f);

    // let mut map = HashMap::new();
    let mut map = BTreeMap::new();

    for line in reader.lines() {
        // Line format: Frankfurt;28.5
        let line = line.unwrap();
        let mut split = line.split(';');
        let mycity = split.next().unwrap().to_string();
        let mytemp = split.next().unwrap().parse::<f32>().unwrap();

        // Check if the city is already in the map
        // if it is, add the temperature to the vector
        // if it is not, create a new vector with the temperature
        let entry: &mut Vec<f32> = map.entry(mycity).or_insert(vec![]);
        entry.push(mytemp);
    }

    // for (city, temperatures) in map.iter() {
    //     println!("{}: {:?}", city, temperatures);
    // }

    // New hashmap to store the minimum, mean and max temperature for each city
    let mut temp_data = BTreeMap::new();

    // Calculate the minimum, maximum and average temperature for each city
    for (city, temperatures) in map.iter() {
        let mut min = f32::MAX;
        let mut max = f32::MIN;
        let mut sum = 0.0;

        for temp in temperatures.iter() {
            if *temp < min {
                min = *temp;
            }
            if *temp > max {
                max = *temp;
            }
            sum += *temp;
        }

        let avg = sum / temperatures.len() as f32;

        // println!("{}: min: {}, max: {}, avg: {}", city, min, max, avg);

        temp_data.insert(city, (min, avg, max));
    }

    println!("{{");
    for (city, (min, avg, max)) in temp_data.iter() {
        println!("  {}={:.1}/{:.1}/{:.1},", city, min, avg, max);
    }
    println!("}}");

}
