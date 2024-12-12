use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Write},
    path::Path,
};

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let result = process_file(&Path::new(&args.get(1).expect("Require a filename")));
    output(std::io::stdout(), &result);
}

fn process_file(path: &Path) -> Vec<(String, f64, f64, f64)> {
    let file = File::open(path).expect("Cannot open file");
    let mut reader = BufReader::new(file);
    let mut cities = HashMap::<String, _>::new();
    let mut line_buffer = Vec::with_capacity(80);
    while let Ok(count) = reader.read_until('\n' as u8, &mut line_buffer) {
        if count == 0 {
            break;
        }
        let len = if line_buffer[count - 1] == '\n' as u8 {
            count - 1
        } else {
            count
        };
        let Some((separator_index, _)) = line_buffer
            .iter()
            .enumerate()
            .find(|(_, c)| **c == ';' as u8)
        else {
            panic!("Invalid line");
        };
        let city = unsafe { std::str::from_utf8_unchecked(&line_buffer[..separator_index]) };
        let measurement_str =
            unsafe { std::str::from_utf8_unchecked(&line_buffer[separator_index + 1..len]) };
        let Ok(measurement) = measurement_str.parse::<f64>() else {
            panic!("Could not parse {:?}", measurement_str.as_bytes());
        };
        if let Some((min, max, sum, count)) = cities.get_mut(city) {
            *min = f64::min(*min, measurement);
            *max = f64::max(*max, measurement);
            *sum += measurement;
            *count += 1;
        } else {
            cities.insert(city.to_string(), (measurement, measurement, measurement, 1));
        }
        line_buffer.clear();
    }
    let mut results = cities
        .into_iter()
        .map(|(city, (min, max, sum, count))| (city.to_string(), min, sum / count as f64, max))
        .collect::<Vec<_>>();
    results.sort_by(|(v1, _, _, _), (v2, _, _, _)| v1.cmp(v2));
    results
}

fn output(mut writer: impl Write, lines: &[(String, f64, f64, f64)]) {
    writeln!(writer, "{{").unwrap();
    for (ref city, min, mean, max) in lines[0..lines.len() - 1].iter() {
        writeln!(writer, "    {city}={min:0.1}/{mean:0.1}/{max:0.1},").unwrap();
    }
    let (ref city, min, mean, max) = lines[lines.len() - 1];
    writeln!(writer, "    {city}={min:0.1}/{mean:0.1}/{max:0.1}").unwrap();
    write!(writer, "}}").unwrap();
}

#[cfg(test)]
mod tests {
    use super::{output, process_file};
    use core::str;
    use googletest::prelude::*;
    use std::{
        fs::read_to_string,
        io::{BufWriter, Write},
        path::Path,
    };

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
    fn outputs_correct_data_with_negative_singleton() -> Result<()> {
        let tempfile = write_content("Arbitrary city;-12.3");

        let result = process_file(tempfile.path());

        verify_that!(
            result,
            unordered_elements_are![(
                eq("Arbitrary city"),
                approx_eq(-12.3),
                approx_eq(-12.3),
                approx_eq(-12.3),
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

    #[test]
    fn outputs_are_sorted_alphabetically() -> Result<()> {
        let tempfile = write_content("C;1\nB;2\nA;3\nD;5");

        let result = process_file(tempfile.path());

        verify_that!(
            result,
            elements_are![
                (eq("A"), anything(), anything(), anything()),
                (eq("B"), anything(), anything(), anything()),
                (eq("C"), anything(), anything(), anything()),
                (eq("D"), anything(), anything(), anything()),
            ]
        )
    }

    #[test]
    fn output_matches_sample() -> Result<()> {
        let mut writer = BufWriter::new(Vec::new());

        let result = process_file(Path::new("../../samples/weather_100.csv"));
        output(&mut writer, &result);

        let expected = read_to_string("../../samples/expected/weather_100.txt").unwrap();
        let actual = String::from_utf8(writer.into_inner().unwrap()).unwrap();
        verify_that!(actual, eq(expected))
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
