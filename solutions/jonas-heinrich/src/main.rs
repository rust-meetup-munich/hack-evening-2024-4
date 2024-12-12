// use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Seek};

use hashbrown::HashMap;

struct Stats {
    min: f64,
    max: f64,
    sum: f64,
    count: usize,
}

impl Stats {
    fn new(temperature: f64) -> Self {
        Self {
            min: temperature,
            max: temperature,
            sum: temperature,
            count: 1,
        }
    }

    fn mean(&self) -> f64 {
        self.sum / self.count as f64
    }

    fn update(&mut self, temp: f64) {
        self.min = self.min.min(temp);
        self.max = self.max.max(temp);
        self.sum += temp;
        self.count += 1;
    }

    fn merge(&mut self, other: &Self) {
        self.min = self.min.min(other.min);
        self.max = self.max.max(other.max);
        self.sum += other.sum;
        self.count += other.count;
    }
}

fn main() -> io::Result<()> {
    assert_eq!(
        env::args().len(),
        2,
        "Usage: {} <input_file.csv>",
        env::args().next().unwrap()
    );

    // Get the file path from command line arguments
    let file_path = &env::args().nth(1).unwrap();

    let cpus = num_cpus::get();
    let file_size = std::fs::metadata(file_path).unwrap().len() as usize;

    let mut threads = Vec::new();
    for cpu in 0..cpus {
        let file_path = file_path.clone();
        threads.push(std::thread::spawn(move || {
            let mut city_data: HashMap<Vec<u8>, Stats> = HashMap::with_capacity(1024);

            let start = (file_size / cpus) * cpu;
            let end = if cpu == cpus - 1 {
                file_size
            } else {
                (file_size / cpus) * (cpu + 1)
            };
            let mut current_offset = start;

            let mut file = File::open(file_path).unwrap();
            file.seek(std::io::SeekFrom::Start(start as u64)).unwrap();

            // Read until the next `\n` character (this skips the CSV header for the first thread)
            let mut reader = BufReader::with_capacity(1024 * 1024, file);
            let mut city_buffer = Vec::with_capacity(1024);
            let mut temperature_buffer = Vec::with_capacity(1024);
            if cpu > 0 {
                current_offset += reader.read_until(b'\n', &mut city_buffer).unwrap();
            }

            // Read the rest of the file
            let mut read_past = false;
            while current_offset < end && !read_past {
                city_buffer.clear();
                temperature_buffer.clear();

                current_offset += reader.read_until(b';', &mut city_buffer).unwrap();
                city_buffer.pop();

                current_offset += reader.read_until(b'\n', &mut temperature_buffer).unwrap();
                if temperature_buffer.last() == Some(&b'\n') {
                    temperature_buffer.pop();
                }

                let temperature: f64 = std::str::from_utf8(&temperature_buffer)
                    .unwrap()
                    .parse()
                    .unwrap();

                if city_data.contains_key(&city_buffer) {
                    let stats = city_data.get_mut(&city_buffer).unwrap();
                    stats.update(temperature);
                } else {
                    let stats = Stats::new(temperature);
                    city_data.insert(city_buffer.clone(), stats);
                }

                if current_offset >= end {
                    read_past = true;
                }
            }

            city_data
        }));
    }

    let mut global = threads.pop().unwrap().join().unwrap();
    for thread in threads {
        let local = thread.join().unwrap();
        for (city, stats) in local {
            if global.contains_key(&city) {
                let global_stats = global.get_mut(&city).unwrap();
                global_stats.merge(&stats);
            } else {
                global.insert(city, stats);
            }
        }
    }
    let global = global;

    let mut keys = Vec::with_capacity(global.keys().len());
    for key in global.keys() {
        keys.push(key);
    }
    keys.sort_unstable();

    let mut results = String::with_capacity(10 * 1024);
    results.push_str("{\n    ");
    let mut separator = "";
    for key in keys {
        let stats = global.get(key).unwrap();
        results.push_str(&format!(
            "{separator}{}={:.1}/{:.1}/{:.1}",
            std::str::from_utf8(key).unwrap(),
            stats.min,
            stats.mean(),
            stats.max
        ));
        separator = ",\n    ";
    }
    results.push_str("\n}");

    // Print the results
    println!("{results}");

    Ok(())
}
