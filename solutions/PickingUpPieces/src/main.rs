use clap::Parser;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Parser,Default,Debug)]
#[clap(version, about = "1 billion row challenge")]
pub struct PickingUpPieces {
    /// Input file: Path to the input file
    #[clap(default_value = "../../samples/weather_100.csv")]
    pub file: String,
}

fn main() {
    let _ = env_logger::try_init();
    let args = PickingUpPieces::parse();

    let mut weather_stations: HashMap<String, Vec<f64>> = HashMap::new();

    if let Ok(lines) = read_lines(&args.file) {
        for line in lines {
            if let Ok(report) = line {
                let parsed_report: Vec<String> = report.split(';').map(|s| s.to_string()).collect();
                weather_stations.entry(parsed_report[0].clone()).or_default().push(parsed_report[1].parse().unwrap());
            }
        }
    }

    let mut result = weather_stations.iter().fold(Vec::new(), |mut acc, (k, v)| { acc.push(Station::new(k, v.clone())); acc });
    result.sort_by(|a, b| a.name.cmp(&b.name));

    println!("{{");
    for station in &result { station.print(); }
    println!("}}");
}


fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug, PartialEq, PartialOrd)]
struct Station {
    name: String,
    min: f64,
    mean: f64,
    max: f64
}

impl Station {
    pub fn new(name: &str, values: Vec<f64>) -> Station {
        let min = values.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let mean = values.iter().sum::<f64>() / values.len() as f64;

        Station {
            name: name.to_string(),
            min,
            mean,
            max
        }
    }

    // Print the results.
    // {
    //     Abha=5.0/18.0/27.4,
    //     Abidjan=15.7/26.0/34.1,
    //     Abéché=12.1/29.4/35.6,
    //     Accra=14.7/26.4/33.1,
    //     Addis Ababa=2.1/16.0/24.3,
    //     Adelaide=4.1/17.3/29.7,
    //     <...>
    //   }
    pub fn print(&self) {
        println!("{}={:.1}/{:.1}/{:.1},", self.name, self.min, self.mean, self.max);
    }
}
