use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{stdout, BufRead, BufReader, BufWriter, Write};

#[derive(Debug, Clone)]
struct Stats {
    avg: f64,
    max: f64,
    min: f64,
    count: i32
}

impl Stats {
    fn new() -> Self {
        Stats {
            avg: 0.0,
            max: -6000000.0,
            min: 6000000.0,
            count: 0
        }
    }
}

fn read_file(file_path: &str) -> Result<(), std::io::Error> {
    let reader = BufReader::new(File::open(file_path)?);
    let lines = reader.lines();

    let mut map: HashMap<String, Stats> = HashMap::new();
    for line in lines {
        let content = line?;
        let l: Vec<String> = content.split(';').map(|s| s.to_string()).collect();
        let key = l[0].clone();
        let value = l[1].parse::<f64>().unwrap();

        let s = map.entry(key).or_insert(Stats::new());

        s.avg += value;
        s.count += 1;
        s.max = s.max.max(value);
        s.min = s.min.min(value);
    }


    
    let mut result: Vec<(String, &Stats)> = vec![];
    for (key, value) in map.iter_mut() {
        value.avg = value.avg / value.count as f64;
        result.push((key.clone(), value));
    }

    result.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

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

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("File path not provided");
    let result = read_file(file_path).expect(" Error reading file");

    
}
