use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{stdout, BufRead, BufReader, BufWriter, Write};
struct Stats {
    avg: f64,
    max: f64,
    min: f64,
}

fn calculate_stats(values: &Vec<f64>) -> Stats {
    let n = values.len();
    let mut sum: f64 = 0.0;
    let mut max: f64 = -6000000.0;
    let mut min: f64 = 6000000.0;
    for i in 0..n {
        sum += values[i];
        if values[i] > max {
            max = values[i];
        }
        if values[i] < min {
            min = values[i];
        }
    }

    let avg = sum / n as f64;

    Stats { avg, max, min }
}

fn read_file(file_path: &str) -> Result<Vec<(String, Stats)>, std::io::Error> {
    let reader = BufReader::new(File::open(file_path)?);
    let lines = reader.lines();

    let mut map: HashMap<String, Vec<f64>> = HashMap::new();
    for line in lines {
        let content = line?.clone();
        let l: Vec<String> = content.split(';').map(|s| s.to_string()).collect();
        map.entry(l[0].clone())
            .or_insert(Vec::new())
            .push(l[1].parse().unwrap());
    }

    let mut output: Vec<(String, Stats)> = vec![];
    for (key, value) in map.iter_mut() {
        let stats = calculate_stats(value);
        output.push((key.clone(), stats));
    }

    output.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    Ok(output)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("File path not provided");
    let result = read_file(file_path).expect(" Error reading file");

    let mut buffer = BufWriter::new(Vec::new());
    writeln!(&mut buffer, "{{").expect("Error writing to buffer");
    for (i, v) in result.iter().enumerate() {
        let (key, stats) = v;

        if i == result.len() - 1 {
            writeln!(
                &mut buffer,
                "    {key}={:.1}/{:.1}/{:.1}",
                stats.min, stats.avg, stats.max
            )
            .expect("Error writing to buffer");
        } else {
            writeln!(
                &mut buffer,
                "    {key}={:.1}/{:.1}/{:.1},",
                stats.min, stats.avg, stats.max
            )
            .expect("Error writing to buffer");
        }
    }
    writeln!(&mut buffer, "}}").expect("Error writing to buffer");
    
    let output = buffer.into_inner().unwrap();
    stdout().write_all(&output).unwrap();
}
