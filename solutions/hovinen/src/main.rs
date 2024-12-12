use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn main() {
    let result = process_file(&Path::new(
        &std::env::args().next().expect("Require a filename"),
    ));
    output(&result);
}

fn process_file(path: &Path) -> Vec<(String, f64, f64, f64)> {
    let file = File::open(path).expect("Cannot open file");
    let reader = BufReader::new(file);
    let mut cities = HashMap::new();
    for (line_number, line) in reader.lines().enumerate() {
        let line = match line {
            Ok(line) => line,
            Err(err) => {
                panic!("Error parsing line {line_number}: {err}")
            }
        };
        let parts = line.split(";").collect::<Vec<_>>();
        let entry = cities
            .entry(parts[0].to_string())
            .or_insert_with(|| (f64::INFINITY, 0.0, 0.0, 0));
        let measurement = parts[1].parse::<f64>().expect("Valid measurement");
        entry.0 = f64::min(entry.0, measurement);
        entry.1 = f64::max(entry.1, measurement);
        entry.2 += measurement;
        entry.3 += 1;
    }
    cities
        .into_iter()
        .map(|(city, (min, max, sum, count))| (city.to_string(), min, sum / count as f64, max))
        .collect()
}

fn output(lines: &[(String, f64, f64, f64)]) {
    for (city, min, mean, max) in lines {
        println!("{city}={min}/{mean}/{max}");
    }
}

#[cfg(test)]
mod tests {
    use super::process_file;
    use googletest::prelude::*;
    use std::io::Write;

    #[test]
    fn outputs_mean_max_min_of_singleton() -> Result<()> {
        let tempfile = write_content("Arbitrary city;12.3");

        let result = process_file(tempfile.path());

        verify_that!(
            result,
            unordered_elements_are![(
                eq("Arbitrary city"),
                approx_eq(12.3),
                approx_eq(12.3),
                approx_eq(12.3),
            )]
        )
    }

    #[test]
    fn outputs_mean_max_min_of_singleton_with_two_measurements() -> Result<()> {
        let tempfile = write_content("Arbitrary city;10.0\nArbitrary city;20.0");

        let result = process_file(tempfile.path());

        verify_that!(
            result,
            unordered_elements_are![(
                eq("Arbitrary city"),
                approx_eq(10.0),
                approx_eq(15.0),
                approx_eq(20.0),
            )]
        )
    }

    #[test]
    fn outputs_mean_max_min_of_two_entries() -> Result<()> {
        let tempfile = write_content("Arbitrary city;12.3\nDifferent city;45.6");

        let result = process_file(tempfile.path());

        verify_that!(
            result,
            unordered_elements_are![
                (
                    eq("Arbitrary city"),
                    approx_eq(12.3),
                    approx_eq(12.3),
                    approx_eq(12.3),
                ),
                (
                    eq("Different city"),
                    approx_eq(45.6),
                    approx_eq(45.6),
                    approx_eq(45.6),
                )
            ]
        )
    }

    fn write_content(content: &str) -> tempfile::NamedTempFile {
        let mut file = tempfile::Builder::new()
            .prefix("test_content_")
            .suffix(".csv")
            .tempfile()
            .unwrap();
        file.write_all(content.as_bytes()).unwrap();
        file
    }
}
